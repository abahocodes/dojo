def sort_array(nums: list[int]) -> list[int]:
    a = list(nums)
    n = len(a)
    buf = [0] * n
    width = 1
    while width < n:
        for lo in range(0, n, 2 * width):
            mid = min(lo + width, n)
            hi = min(lo + 2 * width, n)
            i, j, k = lo, mid, lo
            while i < mid and j < hi:
                if a[i] <= a[j]:
                    buf[k] = a[i]
                    i += 1
                else:
                    buf[k] = a[j]
                    j += 1
                k += 1
            buf[k:hi] = a[i:mid] + a[j:hi]
        a, buf = buf, a
        width *= 2
    return a
