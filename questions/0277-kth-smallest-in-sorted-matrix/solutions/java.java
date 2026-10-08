class Solution {
    public int kthSmallestMatrix(int[][] matrix, int k) {
        int n = matrix.length;
        // Values span up to 2 * 10^9, so do the midpoint arithmetic in long.
        long lo = matrix[0][0], hi = matrix[n - 1][n - 1];
        while (lo < hi) {
            long mid = Math.floorDiv(lo + hi, 2L);
            if (countAtMost(matrix, mid) >= k) hi = mid;
            else lo = mid + 1;
        }
        return (int) lo;
    }

    // Staircase walk from the bottom-left corner.
    private int countAtMost(int[][] matrix, long v) {
        int n = matrix.length;
        int count = 0, row = n - 1, col = 0;
        while (row >= 0 && col < n) {
            if (matrix[row][col] <= v) {
                count += row + 1;
                col++;
            } else {
                row--;
            }
        }
        return count;
    }
}
