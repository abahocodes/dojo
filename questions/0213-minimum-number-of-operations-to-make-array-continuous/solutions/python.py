def min_operations_continuous(nums: list[int]) -> int:
    n = len(nums)
    u = sorted(set(nums))
    best = 0
    j = 0
    for i in range(len(u)):
        while j < len(u) and u[j] <= u[i] + n - 1:
            j += 1
        best = max(best, j - i)
    return n - best
