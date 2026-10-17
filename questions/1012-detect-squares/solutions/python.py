from collections import defaultdict


class DetectSquares:
    def __init__(self):
        self.cnt = defaultdict(int)  # (x, y) -> copies stored
        self.by_x = defaultdict(dict)  # x -> {y: copies stored}

    def add(self, point: list[int]) -> None:
        x, y = point
        self.cnt[(x, y)] += 1
        self.by_x[x][y] = self.by_x[x].get(y, 0) + 1

    def count(self, point: list[int]) -> int:
        qx, qy = point
        total = 0
        # Every point in the query's column fixes the side length.
        for y, c in self.by_x.get(qx, {}).items():
            side = y - qy
            if side == 0:
                continue
            for px in (qx + side, qx - side):
                total += c * self.cnt.get((px, qy), 0) * self.cnt.get((px, y), 0)
        return total


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
