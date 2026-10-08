def height_checker(heights: list[int]) -> int:
    count = [0] * 101
    for h in heights:
        count[h] += 1
    mismatches = 0
    expected = 1
    for h in heights:
        while count[expected] == 0:  # next height in sorted order
            expected += 1
        if h != expected:
            mismatches += 1
        count[expected] -= 1
    return mismatches
