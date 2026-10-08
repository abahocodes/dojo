def maximum_removals(s: str, p: str, removable: list[int]) -> int:
    removed_at = [len(removable)] * len(s)
    for step, i in enumerate(removable):
        removed_at[i] = step

    def survives(k: int) -> bool:
        j = 0
        for i, ch in enumerate(s):
            if j < len(p) and removed_at[i] >= k and ch == p[j]:
                j += 1
        return j == len(p)

    lo, hi = 0, len(removable)
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if survives(mid):
            lo = mid
        else:
            hi = mid - 1
    return lo
