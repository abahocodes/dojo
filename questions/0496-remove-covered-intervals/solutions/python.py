def remove_covered_intervals(intervals: list[list[int]]) -> int:
    # Start ascending; for equal starts the longer interval comes first,
    # so it can cover the shorter ones that follow.
    ordered = sorted(intervals, key=lambda iv: (iv[0], -iv[1]))
    remaining = 0
    max_end = -1
    for _, end in ordered:
        if end > max_end:
            remaining += 1
            max_end = end
    return remaining
