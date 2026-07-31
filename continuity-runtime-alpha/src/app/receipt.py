from datetime import datetime


def generate(event,result):

    return {
        "continuity_receipt": {
            "event_id": event["event_id"],
            "accepted": result["accepted"],
            "scores": result["scores"],
            "timestamp": str(datetime.utcnow()),
            "runtime_version": "alpha-0.1.0"
        }
    }
