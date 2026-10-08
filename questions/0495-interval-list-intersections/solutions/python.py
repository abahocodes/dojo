def interval_intersection(first: list[list[int]], second: list[list[int]]) -> list[list[int]]:
    result = []
    i = j = 0
    while i < len(first) and j < len(second):
        lo = max(first[i][0], second[j][0])
        hi = min(first[i][1], second[j][1])
        if lo <= hi:
            result.append([lo, hi])
        # The interval that ends first cannot meet anything later in the other list.
        if first[i][1] < second[j][1]:
            i += 1
        else:
            j += 1
    return result
