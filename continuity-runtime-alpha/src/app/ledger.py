import json
from datetime import datetime


LEDGER="data/event_ledger.jsonl"


def record(event, receipt):

    entry = {
        "recorded_at": str(datetime.utcnow()),
        "event": event,
        "receipt": receipt
    }

    with open(LEDGER,"a") as f:
        f.write(
            json.dumps(entry)+"\n"
        )

    return entry
