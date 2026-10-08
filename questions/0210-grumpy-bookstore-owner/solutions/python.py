def max_satisfied(customers: list[int], grumpy: list[int], minutes: int) -> int:
    base = sum(c for c, g in zip(customers, grumpy) if g == 0)
    gain = 0
    best = 0
    for i, (c, g) in enumerate(zip(customers, grumpy)):
        if g == 1:
            gain += c
        if i >= minutes and grumpy[i - minutes] == 1:
            gain -= customers[i - minutes]
        best = max(best, gain)
    return base + best
