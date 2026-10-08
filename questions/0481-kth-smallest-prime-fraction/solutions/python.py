def kth_smallest_prime_fraction(arr: list[int], k: int) -> list[int]:
    n = len(arr)
    lo, hi = 0.0, 1.0
    while True:
        mid = (lo + hi) / 2
        count = 0  # fractions strictly below mid
        p, q = 0, 1  # the largest of them
        i = 0
        for j in range(1, n):
            while i < j and arr[i] < mid * arr[j]:
                i += 1
            count += i
            if i > 0 and arr[i - 1] * q > p * arr[j]:
                p, q = arr[i - 1], arr[j]
        if count == k:
            return [p, q]
        if count < k:
            lo = mid
        else:
            hi = mid
