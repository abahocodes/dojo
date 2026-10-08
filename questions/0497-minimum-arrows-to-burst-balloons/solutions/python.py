def find_min_arrow_shots(points: list[list[int]]) -> int:
    # Greedy: shoot each arrow at the right end of the balloon that ends first.
    ordered = sorted(points, key=lambda p: p[1])
    arrows = 1
    pos = ordered[0][1]
    for start, end in ordered:
        if start > pos:
            arrows += 1
            pos = end
    return arrows
