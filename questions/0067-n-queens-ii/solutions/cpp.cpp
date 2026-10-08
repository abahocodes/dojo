class Solution {
public:
    int totalNQueens(int n) {
        full = (1 << n) - 1;
        return place(0, 0, 0);
    }

private:
    int full = 0;

    // cols, diag, anti: bitmasks of the columns attacked in the current row
    int place(int cols, int diag, int anti) {
        if (cols == full) return 1;
        int count = 0;
        int free = full & ~(cols | diag | anti);
        while (free) {
            int bit = free & -free; // lowest free column
            free ^= bit;
            count += place(cols | bit, ((diag | bit) << 1) & full, (anti | bit) >> 1);
        }
        return count;
    }
};
