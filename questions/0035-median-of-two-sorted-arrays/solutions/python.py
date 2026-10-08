def find_median_sorted_arrays(nums1: list[int], nums2: list[int]) -> float:
    if len(nums1) > len(nums2):
        nums1, nums2 = nums2, nums1
    m, n = len(nums1), len(nums2)
    half = (m + n + 1) // 2
    inf = float("inf")
    lo, hi = 0, m
    while lo <= hi:
        i = (lo + hi) // 2
        j = half - i
        left1 = nums1[i - 1] if i > 0 else -inf
        right1 = nums1[i] if i < m else inf
        left2 = nums2[j - 1] if j > 0 else -inf
        right2 = nums2[j] if j < n else inf
        if left1 > right2:
            hi = i - 1
        elif left2 > right1:
            lo = i + 1
        else:
            if (m + n) % 2 == 1:
                return float(max(left1, left2))
            return (max(left1, left2) + min(right1, right2)) / 2
    return 0.0
