def find_max_length(nums: list[int]) -> int:
    first = {0: -1}
    balance = 0
    best = 0
    for i, x in enumerate(nums):
        balance += 1 if x == 1 else -1
        if balance in first:
            best = max(best, i - first[balance])
        else:
            first[balance] = i
    return best
