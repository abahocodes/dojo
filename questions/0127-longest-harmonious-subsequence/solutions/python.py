from collections import Counter


def find_lhs(nums: list[int]) -> int:
    count = Counter(nums)
    best = 0
    for x, c in count.items():
        if x + 1 in count:
            best = max(best, c + count[x + 1])
    return best
