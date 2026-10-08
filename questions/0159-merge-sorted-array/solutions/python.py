def merge_sorted(nums1: list[int], nums2: list[int]) -> list[int]:
    m, n = len(nums1), len(nums2)
    out = nums1 + [0] * n
    i, j, w = m - 1, n - 1, m + n - 1
    while j >= 0:
        if i >= 0 and out[i] > nums2[j]:
            out[w] = out[i]
            i -= 1
        else:
            out[w] = nums2[j]
            j -= 1
        w -= 1
    return out
