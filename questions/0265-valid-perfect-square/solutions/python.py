def is_perfect_square(num: int) -> bool:
    lo, hi = 1, min(num, 1 << 26)
    while lo <= hi:
        mid = (lo + hi) // 2
        square = mid * mid
        if square == num:
            return True
        if square < num:
            lo = mid + 1
        else:
            hi = mid - 1
    return False
