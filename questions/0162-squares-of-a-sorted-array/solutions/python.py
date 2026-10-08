def sorted_squares(nums: list[int]) -> list[int]:
    n = len(nums)
    out = [0] * n
    lo, hi = 0, n - 1
    for w in range(n - 1, -1, -1):
        if abs(nums[lo]) > abs(nums[hi]):
            out[w] = nums[lo] * nums[lo]
            lo += 1
        else:
            out[w] = nums[hi] * nums[hi]
            hi -= 1
    return out
