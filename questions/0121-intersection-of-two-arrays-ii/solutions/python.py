def intersect(nums1: list[int], nums2: list[int]) -> list[int]:
    counts = [0] * 1001
    for x in nums1:
        counts[x] += 1
    out = []
    for x in nums2:
        if counts[x] > 0:
            counts[x] -= 1
            out.append(x)
    return out
