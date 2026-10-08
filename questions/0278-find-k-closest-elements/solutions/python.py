def find_closest_elements(arr: list[int], k: int, x: int) -> list[int]:
    # Binary search for the left edge of the best window arr[left:left + k].
    lo, hi = 0, len(arr) - k
    while lo < hi:
        mid = (lo + hi) // 2
        if x - arr[mid] > arr[mid + k] - x:
            lo = mid + 1
        else:
            hi = mid
    return arr[lo:lo + k]
