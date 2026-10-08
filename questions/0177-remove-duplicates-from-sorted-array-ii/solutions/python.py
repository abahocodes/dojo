def remove_duplicates_keep_two(nums: list[int]) -> list[int]:
    a = list(nums)
    k = 0
    for x in a:
        if k < 2 or a[k - 2] != x:
            a[k] = x
            k += 1
    return a[:k]
