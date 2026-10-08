def remove_interval(intervals: list[list[int]], to_be_removed: list[int]) -> list[list[int]]:
    cut_lo, cut_hi = to_be_removed
    result = []
    for a, b in intervals:
        if b <= cut_lo or a >= cut_hi:
            result.append([a, b])  # untouched
            continue
        if a < cut_lo:
            result.append([a, cut_lo])  # piece left of the cut
        if b > cut_hi:
            result.append([cut_hi, b])  # piece right of the cut
    return result
