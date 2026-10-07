def max_profit(prices: list[int]) -> int:
    lowest = prices[0]
    best = 0
    for p in prices:
        best = max(best, p - lowest)
        lowest = min(lowest, p)
    return best
