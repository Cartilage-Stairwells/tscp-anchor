#!/usr/bin/env python3
"""
OWSL — Oracle Witness Status Layer
Daemon process: samples kernel entropy pool health, maintains bit accounting
within a sliding window, detects anomalies, and writes atomic status updates
to ~/.tscp/owsl_status.json for consumption by owsl_bridge.rs.

Extension points marked with TODO(owsl-zk): for future ZK/proof-specific
bit accounting (Fiat-Shamir challenge entropy, witness blinding, etc.)
"""

import json
import os
import signal
import sys
import time
import logging
from dataclasses import dataclass, asdict, field
from typing import List, Optional
from collections import deque

# ── Paths ─────────────────────────────────────────────────────────────────────

OWSL_STATUS_PATH = os.path.expanduser("~/.tscp/owsl_status.json")
OWSL_STATUS_DIR  = os.path.dirname(OWSL_STATUS_PATH)
ENTROPY_AVAIL    = "/proc/sys/kernel/random/entropy_avail"
ENTROPY_POOLSIZE = "/proc/sys/kernel/random/poolsize"

# ── Thresholds ─────────────────────────────────────────────────────────────────

ENTROPY_CRITICAL_BITS    = 64    # Below this → CRITICAL / HALT
ENTROPY_WARNING_BITS     = 256   # Below this → WARNING / LOG
ENTROPY_HEALTHY_BITS     = 512   # Comfortable floor
WRITE_INTERVAL_SECONDS   = 30    # How often the daemon writes status
WINDOW_DURATION_SECONDS  = 3600  # Sliding window for bit accounting
ANOMALY_HISTORY_LIMIT    = 50    # Max anomaly records kept in memory
STALE_THRESHOLD_SECONDS  = 90    # Bridge rejects after 60s; we write every 30s

# ── Logging ───────────────────────────────────────────────────────────────────

logging.basicConfig(
    level=logging.INFO,
    format="[OWSL %(levelname)s %(asctime)s] %(message)s",
    datefmt="%Y-%m-%dT%H:%M:%S",
)
log = logging.getLogger("owsl")

# ── Data model ────────────────────────────────────────────────────────────────

@dataclass
class OWSLStatus:
    timestamp:      float
    status:         str          # SAFE | WARNING | CRITICAL
    action:         str          # COMMIT | LOG | HALT
    round:          int          # Monotonic write counter
    bits_consumed:  int          # Estimated bits drawn in current window
    bits_remaining: int          # Estimated bits available above safety floor
    anomalies:      List[str]    # Recent anomaly descriptions
    frame_count:    int          # Number of samples taken in current window
    window_start:   float
    window_end:     float
    checksum_valid: bool         # Always True for kernel-sourced data; hook for future integrity checks

    def to_json(self) -> str:
        return json.dumps(asdict(self), indent=2)

    @classmethod
    def from_json(cls, data: str) -> "OWSLStatus":
        return cls(**json.loads(data))

    def permits_verification(self) -> bool:
        return self.status != "CRITICAL" and self.action != "HALT"


# ── Entropy sampling ──────────────────────────────────────────────────────────

def read_entropy_avail() -> Optional[int]:
    """Read current kernel entropy pool level in bits."""
    try:
        with open(ENTROPY_AVAIL, "r") as f:
            return int(f.read().strip())
    except (OSError, ValueError) as e:
        log.error("Cannot read entropy_avail: %s", e)
        return None


def read_pool_size() -> int:
    """Read maximum kernel entropy pool size in bits (usually 4096)."""
    try:
        with open(ENTROPY_POOLSIZE, "r") as f:
            return int(f.read().strip())
    except (OSError, ValueError):
        return 4096  # Safe default


# ── Anomaly detection ─────────────────────────────────────────────────────────

@dataclass
class EntropyFrame:
    timestamp: float
    avail_bits: int


class AnomalyDetector:
    def __init__(self, window_seconds: int = WINDOW_DURATION_SECONDS):
        self.window_seconds = window_seconds
        self.frames: deque = deque(maxlen=1000)
        self.anomalies: deque = deque(maxlen=ANOMALY_HISTORY_LIMIT)

    def record(self, avail_bits: int) -> None:
        now = time.time()
        self.frames.append(EntropyFrame(timestamp=now, avail_bits=avail_bits))
        self._expire_old_frames(now)
        self._check(avail_bits, now)

    def _expire_old_frames(self, now: float) -> None:
        cutoff = now - self.window_seconds
        while self.frames and self.frames[0].timestamp < cutoff:
            self.frames.popleft()

    def _check(self, avail_bits: int, now: float) -> None:
        # Hard floor breach
        if avail_bits < ENTROPY_CRITICAL_BITS:
            self._record_anomaly(
                now,
                f"CRITICAL: entropy_avail={avail_bits} < floor={ENTROPY_CRITICAL_BITS}"
            )
            return

        if avail_bits < ENTROPY_WARNING_BITS:
            self._record_anomaly(
                now,
                f"WARNING: entropy_avail={avail_bits} < threshold={ENTROPY_WARNING_BITS}"
            )
            return

        # Sudden drop detection: compare to recent baseline
        if len(self.frames) >= 3:
            recent = list(self.frames)[-3:]
            baseline = sum(f.avail_bits for f in recent[:-1]) / max(len(recent) - 1, 1)
            current  = recent[-1].avail_bits
            if baseline > 0 and current < baseline * 0.5:
                self._record_anomaly(
                    now,
                    f"RATE_DROP: entropy fell from ~{int(baseline)} to {current} bits (>50% drop)"
                )

        # TODO(owsl-zk): Add ZK proof randomness consumption accounting here.
        # When the Plonky3 prover draws entropy for Fiat-Shamir challenges,
        # hook into this detector to record bits_consumed per proof round.

    def _record_anomaly(self, timestamp: float, description: str) -> None:
        entry = f"[{time.strftime('%Y-%m-%dT%H:%M:%S', time.localtime(timestamp))}] {description}"
        self.anomalies.append(entry)
        log.warning("Anomaly recorded: %s", description)

    def recent_anomalies(self, limit: int = 10) -> List[str]:
        return list(self.anomalies)[-limit:]

    def bits_consumed_in_window(self) -> int:
        """
        Estimate bits consumed as the drop from max observed to current level
        within the window. This is a lower-bound heuristic; replace with
        direct accounting when ZK hooks are wired.
        # TODO(owsl-zk): Replace with exact consumption counters from prover.
        """
        if len(self.frames) < 2:
            return 0
        max_seen = max(f.avail_bits for f in self.frames)
        current  = self.frames[-1].avail_bits
        return max(0, max_seen - current)

    def window_bounds(self):
        if not self.frames:
            now = time.time()
            return now - self.window_seconds, now
        return self.frames[0].timestamp, self.frames[-1].timestamp


# ── Status classification ─────────────────────────────────────────────────────

def classify(avail_bits: int, anomalies: List[str]) -> tuple:
    """Return (status, action) based on current entropy level and anomaly list."""
    critical_anomaly = any("CRITICAL" in a for a in anomalies[-5:])

    if avail_bits < ENTROPY_CRITICAL_BITS or critical_anomaly:
        return "CRITICAL", "HALT"
    if avail_bits < ENTROPY_WARNING_BITS:
        return "WARNING", "LOG"
    return "SAFE", "COMMIT"


# ── Atomic writer ─────────────────────────────────────────────────────────────

class OWSLIPCWriter:
    def __init__(self, path: str = OWSL_STATUS_PATH):
        self.path      = path
        self._tmp_path = path + ".tmp"
        os.makedirs(os.path.dirname(path), mode=0o700, exist_ok=True)

    def write(self, status: OWSLStatus) -> None:
        with open(self._tmp_path, "w", encoding="utf-8") as f:
            f.write(status.to_json())
            f.flush()
            os.fsync(f.fileno())
        os.replace(self._tmp_path, self.path)
        log.debug("Status written: %s / %s (entropy=%d)",
                  status.status, status.action, status.bits_remaining)

    def read(self) -> Optional[OWSLStatus]:
        if not os.path.exists(self.path):
            return None
        try:
            with open(self.path, "r", encoding="utf-8") as f:
                return OWSLStatus.from_json(f.read())
        except (json.JSONDecodeError, OSError, TypeError) as e:
            log.error("Read error: %s", e)
            return None


# ── Daemon ────────────────────────────────────────────────────────────────────

class OWSLDaemon:
    def __init__(self):
        self.writer   = OWSLIPCWriter()
        self.detector = AnomalyDetector()
        self.round    = 0
        self._running = True
        self._pool_size = read_pool_size()
        signal.signal(signal.SIGTERM, self._handle_signal)
        signal.signal(signal.SIGINT,  self._handle_signal)

    def _handle_signal(self, signum, frame):
        log.info("Caught signal %d — shutting down.", signum)
        self._running = False

    def _sample_and_write(self) -> None:
        avail = read_entropy_avail()

        if avail is None:
            # Kernel read failed — write a WARNING so the bridge doesn't block forever
            avail = 0
            self.detector._record_anomaly(time.time(), "CRITICAL: entropy_avail unreadable")

        self.detector.record(avail)
        self.round += 1

        anomalies     = self.detector.recent_anomalies()
        status, action = classify(avail, anomalies)
        bits_consumed = self.detector.bits_consumed_in_window()
        bits_remaining = max(0, avail - ENTROPY_CRITICAL_BITS)
        w_start, w_end = self.detector.window_bounds()

        owsl_status = OWSLStatus(
            timestamp      = time.time(),
            status         = status,
            action         = action,
            round          = self.round,
            bits_consumed  = bits_consumed,
            bits_remaining = bits_remaining,
            anomalies      = anomalies,
            frame_count    = len(self.detector.frames),
            window_start   = w_start,
            window_end     = w_end,
            checksum_valid = True,  # TODO(owsl-zk): compute HMAC over payload when ledger is wired
        )

        self.writer.write(owsl_status)

        if status != "SAFE":
            log.warning("Status: %s / %s | entropy_avail=%d bits_remaining=%d",
                        status, action, avail, bits_remaining)
        else:
            log.info("Status: SAFE | entropy_avail=%d bits_remaining=%d round=%d",
                     avail, bits_remaining, self.round)

    def run(self) -> None:
        log.info("OWSL daemon starting. pool_size=%d write_interval=%ds",
                 self._pool_size, WRITE_INTERVAL_SECONDS)

        # Write an initial status immediately so the bridge doesn't block on cold start
        self._sample_and_write()

        while self._running:
            time.sleep(WRITE_INTERVAL_SECONDS)
            if self._running:
                self._sample_and_write()

        log.info("OWSL daemon stopped cleanly at round %d.", self.round)


# ── CLI ───────────────────────────────────────────────────────────────────────

def cmd_run():
    OWSLDaemon().run()


def cmd_status():
    writer = OWSLIPCWriter()
    s = writer.read()
    if s is None:
        print("No status file found. Is the OWSL daemon running?")
        sys.exit(1)
    age = time.time() - s.timestamp
    print(f"Status:         {s.status} / {s.action}")
    print(f"Round:          {s.round}")
    print(f"Entropy avail:  {s.bits_remaining + ENTROPY_CRITICAL_BITS} bits (est)")
    print(f"Bits remaining: {s.bits_remaining}")
    print(f"Bits consumed:  {s.bits_consumed} (this window)")
    print(f"Frame count:    {s.frame_count}")
    print(f"Age:            {age:.1f}s")
    print(f"Permits verify: {s.permits_verification()}")
    if s.anomalies:
        print("Anomalies:")
        for a in s.anomalies:
            print(f"  {a}")


def cmd_test():
    """One-shot: write a single SAFE status and exit. For CI / cold-start bootstrap."""
    writer = OWSLIPCWriter()
    avail  = read_entropy_avail() or ENTROPY_HEALTHY_BITS
    status = OWSLStatus(
        timestamp      = time.time(),
        status         = "SAFE",
        action         = "COMMIT",
        round          = 0,
        bits_consumed  = 0,
        bits_remaining = max(0, avail - ENTROPY_CRITICAL_BITS),
        anomalies      = [],
        frame_count    = 1,
        window_start   = time.time() - WINDOW_DURATION_SECONDS,
        window_end     = time.time(),
        checksum_valid = True,
    )
    writer.write(status)
    print(f"[OWSL] Bootstrap status written to {OWSL_STATUS_PATH}")
    print(f"[OWSL] entropy_avail={avail} bits_remaining={status.bits_remaining}")


if __name__ == "__main__":
    cmds = {"run": cmd_run, "status": cmd_status, "test": cmd_test}
    if len(sys.argv) < 2 or sys.argv[1] not in cmds:
        print(f"Usage: owsl_ipc.py [{' | '.join(cmds)}]")
        sys.exit(1)
    cmds[sys.argv[1]]()
