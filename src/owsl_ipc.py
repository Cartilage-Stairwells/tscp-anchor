#!/usr/bin/env python3
import json
import os
import time
from dataclasses import dataclass, asdict
from typing import List, Optional

OWSL_STATUS_PATH = os.path.expanduser("~/.tscp/owsl_status.json")
OWSL_STATUS_DIR = os.path.dirname(OWSL_STATUS_PATH)
os.makedirs(OWSL_STATUS_DIR, mode=0o700, exist_ok=True)

@dataclass
class OWSLStatus:
    timestamp: float
    status: str
    action: str
    round: int
    bits_consumed: int
    bits_remaining: int
    anomalies: List[str]
    frame_count: int
    window_start: float
    window_end: float
    checksum_valid: bool

    def to_json(self) -> str:
        return json.dumps(asdict(self), indent=2)

    @classmethod
    def from_json(cls, data: str) -> "OWSLStatus":
        return cls(**json.loads(data))

    def permits_verification(self) -> bool:
        return self.status != "CRITICAL" and self.action != "HALT"

class OWSLIPCWriter:
    def __init__(self, path: str = OWSL_STATUS_PATH):
        self.path = path
        self._temp_path = path + ".tmp"

    def write(self, status: OWSLStatus) -> None:
        with open(self._temp_path, "w", encoding="utf-8") as f:
            f.write(status.to_json())
            f.flush()
            os.fsync(f.fileno())
        os.replace(self._temp_path, self.path)

    def read(self) -> Optional[OWSLStatus]:
        if not os.path.exists(self.path):
            return None
        try:
            with open(self.path, "r", encoding="utf-8") as f:
                return OWSLStatus.from_json(f.read())
        except (json.JSONDecodeError, OSError) as e:
            print(f"[OWSL-IPC] Read error: {e}")
            return None

def generate_test_status(status: str = "SAFE") -> OWSLStatus:
    return OWSLStatus(
        timestamp=time.time(),
        status=status,
        action="COMMIT" if status == "SAFE" else ("LOG" if status == "WARNING" else "HALT"),
        round=42,
        bits_consumed=32,
        bits_remaining=96,
        anomalies=[],
        frame_count=10,
        window_start=time.time() - 3600,
        window_end=time.time(),
        checksum_valid=True
    )

if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1 and sys.argv[1] == "--test":
        writer = OWSLIPCWriter()
        for s in ["SAFE", "WARNING", "CRITICAL"]:
            print(f"[OWSL-IPC] Writing {s} status...")
            writer.write(generate_test_status(s))
            time.sleep(1)
        print(f"[OWSL-IPC] Status files written to {OWSL_STATUS_PATH}")
    else:
        print("[OWSL-IPC] Usage: python owsl_ipc.py --test")
