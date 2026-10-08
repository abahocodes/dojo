MOD = 10**9 + 7


def num_subseq(nums: list[int], target: int) -> int:
    a = sorted(nums)
    n = len(a)
    pow2 = [1] * n
    for i in range(1, n):
        pow2[i] = pow2[i - 1] * 2 % MOD
    total = 0
    lo, hi = 0, n - 1
    while lo <= hi:
        if a[lo] + a[hi] <= target:
            total = (total + pow2[hi - lo]) % MOD
            lo += 1
        else:
            hi -= 1
    return total
