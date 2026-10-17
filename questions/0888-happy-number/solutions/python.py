def is_happy(n: int) -> bool:
    # The digit-square sequence either reaches 1 or falls into a cycle.
    # Floyd's tortoise and hare finds out without remembering every value.
    def step(v: int) -> int:
        total = 0
        while v > 0:
            v, d = divmod(v, 10)
            total += d * d
        return total

    slow, fast = n, step(n)
    while fast != 1 and slow != fast:
        slow = step(slow)
        fast = step(step(fast))
    return fast == 1
