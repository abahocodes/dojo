def four_sum_count(nums1: list[int], nums2: list[int], nums3: list[int], nums4: list[int]) -> int:
    sums = {}
    for a in nums1:
        for b in nums2:
            sums[a + b] = sums.get(a + b, 0) + 1
    count = 0
    for c in nums3:
        for d in nums4:
            count += sums.get(-(c + d), 0)
    return count
