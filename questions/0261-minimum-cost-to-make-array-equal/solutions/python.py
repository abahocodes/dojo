def min_cost(nums: list[int], cost: list[int]) -> int:
    pairs = sorted(zip(nums, cost))
    total = sum(cost)
    acc = 0
    target = pairs[0][0]
    for value, weight in pairs:
        acc += weight
        if 2 * acc >= total:
            target = value
            break
    return sum(w * abs(v - target) for v, w in pairs)
