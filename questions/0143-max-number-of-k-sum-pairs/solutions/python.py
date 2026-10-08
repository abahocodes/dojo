def max_operations(nums: list[int], k: int) -> int:
    waiting = {}
    ops = 0
    for x in nums:
        partner = k - x
        if waiting.get(partner, 0) > 0:
            waiting[partner] -= 1
            ops += 1
        else:
            waiting[x] = waiting.get(x, 0) + 1
    return ops
