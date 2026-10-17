# 1012. Detect Squares (medium)
# /show to reread the problem · /test runs visible tests · /submit when done

class DetectSquares:
    def __init__(self):
        pass

    def add(self, point: list[int]) -> None:
        pass

    def count(self, point: list[int]) -> int:
        return 0


# dojo calls this to run the tests; you don't need to change it.
def run_detect_squares(ops: list[str], points: list[list[int]]) -> list[int]:
    ds = DetectSquares()
    out = []
    for op, p in zip(ops, points):
        if op == "add":
            ds.add(p)
        else:
            out.append(ds.count(p))
    return out
