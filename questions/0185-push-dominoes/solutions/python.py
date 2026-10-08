def push_dominoes(dominoes: str) -> str:
    n = len(dominoes)
    res = list(dominoes)
    marks = [(-1, "L")] + [(i, c) for i, c in enumerate(dominoes) if c != "."] + [(n, "R")]
    for (i, x), (j, y) in zip(marks, marks[1:]):
        if x == y:
            for k in range(i + 1, j):
                res[k] = x
        elif x == "R" and y == "L":
            lo, hi = i + 1, j - 1
            while lo < hi:
                res[lo] = "R"
                res[hi] = "L"
                lo += 1
                hi -= 1
    return "".join(res)
