def k_weakest_rows(mat: list[list[int]], k: int) -> list[int]:
    def soldiers(row: list[int]) -> int:
        # Rows are 1s then 0s: binary search for the first 0.
        lo, hi = 0, len(row)
        while lo < hi:
            mid = (lo + hi) // 2
            if row[mid] == 1:
                lo = mid + 1
            else:
                hi = mid
        return lo

    ranked = sorted(range(len(mat)), key=lambda i: (soldiers(mat[i]), i))
    return ranked[:k]
