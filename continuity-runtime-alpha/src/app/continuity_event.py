import uuid
from datetime import datetime


def create_event(data):

    return {
        "event_id": str(uuid.uuid4()),
        "artifact_id": data.get("artifact_id"),
        "transformation": data.get("transformation"),
        "timestamp": str(datetime.utcnow()),
        "evidence": data.get("evidence", {})
    }
