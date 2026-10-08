def min_moves_complementary(nums: list[int], limit: int) -> int:
    n = len(nums)
    # delta[t] = change in total moves when the target sum goes from t-1 to t
    delta = [0] * (2 * limit + 2)
    for i in range(n // 2):
        a, b = nums[i], nums[n - 1 - i]
        lo, hi = min(a, b), max(a, b)
        delta[2] += 2               # by default a pair needs two moves
        delta[lo + 1] -= 1          # from lo + 1 one move suffices ...
        delta[hi + limit + 1] += 1  # ... up to hi + limit
        delta[a + b] -= 1           # exactly a + b needs no move
        delta[a + b + 1] += 1
    best = n
    moves = 0
    for t in range(2, 2 * limit + 1):
        moves += delta[t]
        best = min(best, moves)
    return best
