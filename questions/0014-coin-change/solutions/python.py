def coin_change(coins: list[int], amount: int) -> int:
    INF = amount + 1
    best = [0] + [INF] * amount
    for total in range(1, amount + 1):
        for coin in coins:
            if coin <= total and best[total - coin] + 1 < best[total]:
                best[total] = best[total - coin] + 1
    return best[amount] if best[amount] != INF else -1
