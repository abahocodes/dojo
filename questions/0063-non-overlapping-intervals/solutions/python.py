def erase_overlap_intervals(intervals: list[list[int]]) -> int:
    removed = 0
    last_end = float("-inf")
    # keeping the interval that ends first leaves the most room for the rest
    for start, end in sorted(intervals, key=lambda iv: iv[1]):
        if start >= last_end:
            last_end = end
        else:
            removed += 1
    return removed
