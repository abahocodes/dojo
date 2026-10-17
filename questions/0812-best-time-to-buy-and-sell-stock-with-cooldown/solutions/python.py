def max_profit(prices: list[int]) -> int:
    # Best profit at the end of each day, by state:
    # hold = owning a share, sold = sold today, rest = no share and free to buy.
    hold, sold, rest = float("-inf"), 0, 0
    for p in prices:
        hold, sold, rest = max(hold, rest - p), hold + p, max(rest, sold)
    return max(sold, rest)
