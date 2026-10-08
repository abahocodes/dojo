def max_sub_array_len(nums: list[int], k: int) -> int:
    first = {0: -1}
    prefix = 0
    best = 0
    for i, x in enumerate(nums):
        prefix += x
        j = first.get(prefix - k)
        if j is not None:
            best = max(best, i - j)
        if prefix not in first:
            first[prefix] = i
    return best
