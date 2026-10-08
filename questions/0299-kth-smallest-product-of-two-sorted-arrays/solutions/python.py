from bisect import bisect_left, bisect_right


def kth_smallest_product(nums1: list[int], nums2: list[int], k: int) -> int:
    n2 = len(nums2)

    def count_at_most(x: int) -> int:
        total = 0
        for a in nums1:
            if a > 0:
                # a * b <= x  <=>  b <= floor(x / a)
                total += bisect_right(nums2, x // a)
            elif a < 0:
                # a * b <= x  <=>  b >= ceil(x / a)
                total += n2 - bisect_left(nums2, -(-x // a))
            elif x >= 0:
                total += n2
        return total

    corners = [nums1[0] * nums2[0], nums1[0] * nums2[-1],
               nums1[-1] * nums2[0], nums1[-1] * nums2[-1]]
    lo, hi = min(corners), max(corners)
    while lo < hi:
        mid = (lo + hi) // 2
        if count_at_most(mid) >= k:
            hi = mid
        else:
            lo = mid + 1
    return lo
