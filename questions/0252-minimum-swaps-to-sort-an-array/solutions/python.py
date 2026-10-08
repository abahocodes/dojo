def min_swaps_to_sort(nums: list[int]) -> int:
    n = len(nums)
    # order[k] = index of the k-th smallest value
    order = sorted(range(n), key=nums.__getitem__)
    seen = [False] * n
    swaps = 0
    for i in range(n):
        length = 0
        j = i
        while not seen[j]:
            seen[j] = True
            j = order[j]
            length += 1
        if length:
            swaps += length - 1
    return swaps
