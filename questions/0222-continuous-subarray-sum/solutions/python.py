def check_subarray_sum(nums: list[int], k: int) -> bool:
    first = {0: -1}
    rem = 0
    for i, x in enumerate(nums):
        rem = (rem + x) % k
        if rem in first:
            if i - first[rem] >= 2:
                return True
        else:
            first[rem] = i
    return False
