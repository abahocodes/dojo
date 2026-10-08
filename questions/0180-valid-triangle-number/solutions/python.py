def triangle_number(nums: list[int]) -> int:
    a = sorted(nums)
    count = 0
    for k in range(len(a) - 1, 1, -1):
        i, j = 0, k - 1
        while i < j:
            if a[i] + a[j] > a[k]:
                count += j - i
                j -= 1
            else:
                i += 1
    return count
