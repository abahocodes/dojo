def kth_smallest_matrix(matrix: list[list[int]], k: int) -> int:
    n = len(matrix)

    def count_at_most(v: int) -> int:
        # Staircase walk from the bottom-left corner.
        count, row, col = 0, n - 1, 0
        while row >= 0 and col < n:
            if matrix[row][col] <= v:
                count += row + 1
                col += 1
            else:
                row -= 1
        return count

    lo, hi = matrix[0][0], matrix[n - 1][n - 1]
    while lo < hi:
        mid = (lo + hi) // 2
        if count_at_most(mid) >= k:
            hi = mid
        else:
            lo = mid + 1
    return lo
