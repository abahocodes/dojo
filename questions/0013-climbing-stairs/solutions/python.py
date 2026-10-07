def climb_stairs(n: int) -> int:
    prev, curr = 1, 1  # ways to reach step 0 and step 1
    for _ in range(n - 1):
        prev, curr = curr, prev + curr
    return curr
