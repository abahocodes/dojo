def max_ice_cream(costs: list[int], coins: int) -> int:
    count = [0] * (max(costs) + 1)
    for c in costs:
        count[c] += 1
    bought = 0
    for price in range(1, len(count)):
        if count[price] == 0:
            continue
        take = min(count[price], coins // price)
        bought += take
        coins -= take * price
        if take < count[price]:
            break
    return bought
