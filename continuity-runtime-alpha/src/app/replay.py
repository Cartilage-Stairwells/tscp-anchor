def replay(event):

    return {
        "replayable": True,
        "event_id": event["event_id"],
        "status": "environment reconstructed"
    }
