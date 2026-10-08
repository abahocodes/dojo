def range_sums(nums: list[int], queries: list[list[int]]) -> list[int]:
    prefix = [0]
    for x in nums:
        prefix.append(prefix[-1] + x)
    return [prefix[r + 1] - prefix[l] for l, r in queries]
