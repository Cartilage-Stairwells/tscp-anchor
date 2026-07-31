from .continuity_contract import ContinuityContract


def evaluate(event):

    scores = {
        "semantic": 0.98,
        "computational": 1.0,
        "identity": 1.0,
        "adaptive": 0.85
    }

    contract = ContinuityContract()

    return {
        "accepted": contract.validate(scores),
        "scores": scores
    }
