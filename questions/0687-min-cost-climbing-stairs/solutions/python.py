def min_cost_climbing_stairs(cost: list[int]) -> int:
    # a, b = cheapest way to stand on step i-2, i-1 (steps 0 and 1 are free starts).
    a, b = 0, 0
    for i in range(2, len(cost) + 1):
        a, b = b, min(b + cost[i - 1], a + cost[i - 2])
    return b
