class ContinuityContract:

    thresholds = {
        "semantic": 0.95,
        "computational": 1.0,
        "identity": 1.0,
        "adaptive": 0.80
    }

    def validate(self, scores):

        for domain, minimum in self.thresholds.items():
            if scores.get(domain, 0) < minimum:
                return False

        return True
