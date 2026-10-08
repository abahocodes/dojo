def minimum_abs_difference(arr: list[int]) -> list[list[int]]:
    a = sorted(arr)
    best = min(a[i + 1] - a[i] for i in range(len(a) - 1))
    return [[a[i], a[i + 1]] for i in range(len(a) - 1) if a[i + 1] - a[i] == best]
