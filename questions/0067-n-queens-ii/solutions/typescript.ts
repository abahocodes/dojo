function totalNQueens(n: number): number {
    const full = (1 << n) - 1;

    // cols, diag, anti: bitmasks of the columns attacked in the current row
    const place = (cols: number, diag: number, anti: number): number => {
        if (cols === full) return 1;
        let count = 0;
        let free = full & ~(cols | diag | anti);
        while (free) {
            const bit = free & -free; // lowest free column
            free ^= bit;
            count += place(cols | bit, ((diag | bit) << 1) & full, (anti | bit) >> 1);
        }
        return count;
    };

    return place(0, 0, 0);
}
