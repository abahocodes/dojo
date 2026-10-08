import random


def min_moves_to_equal(nums: list[int]) -> int:
    a = list(nums)
    k = len(a) // 2
    lo, hi = 0, len(a) - 1
    while lo < hi:
        pivot = a[random.randint(lo, hi)]
        # Three-way partition: a[lo:lt] < pivot, a[lt:gt+1] == pivot, a[gt+1:hi+1] > pivot
        lt, i, gt = lo, lo, hi
        while i <= gt:
            if a[i] < pivot:
                a[lt], a[i] = a[i], a[lt]
                lt += 1
                i += 1
            elif a[i] > pivot:
                a[i], a[gt] = a[gt], a[i]
                gt -= 1
            else:
                i += 1
        if k < lt:
            hi = lt - 1
        elif k > gt:
            lo = gt + 1
        else:
            break
    median = a[k]
    return sum(abs(v - median) for v in a)
