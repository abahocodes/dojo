def count_combinations(coins: list[int], amount: int) -> int:
    ways = [1] + [0] * amount
    for c in coins:
        for x in range(c, amount + 1):
            ways[x] += ways[x - c]
    return ways[amount]
