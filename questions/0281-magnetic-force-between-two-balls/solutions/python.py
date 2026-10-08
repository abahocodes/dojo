def max_min_distance(position: list[int], m: int) -> int:
    pos = sorted(position)

    def fits(gap: int) -> bool:
        # Greedily drop a ball in the leftmost basket at least `gap` past the last one.
        placed, last = 1, pos[0]
        for p in pos[1:]:
            if p - last >= gap:
                placed += 1
                last = p
                if placed == m:
                    return True
        return False

    lo, hi = 1, (pos[-1] - pos[0]) // (m - 1)
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if fits(mid):
            lo = mid
        else:
            hi = mid - 1
    return lo
