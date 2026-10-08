def longest_square_streak(nums: list[int]) -> int:
    present = set(nums)
    best = -1
    for x in present:
        length = 1
        cur = x
        while cur * cur in present:
            cur *= cur
            length += 1
        if length >= 2 and length > best:
            best = length
    return best
