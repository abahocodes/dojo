def total_n_queens(n: int) -> int:
    full = (1 << n) - 1

    # cols, diag, anti: bitmasks of the columns attacked in the current row
    def place(cols, diag, anti):
        if cols == full:
            return 1
        count = 0
        free = full & ~(cols | diag | anti)
        while free:
            bit = free & -free  # lowest free column
            free ^= bit
            count += place(cols | bit, ((diag | bit) << 1) & full, (anti | bit) >> 1)
        return count

    return place(0, 0, 0)
