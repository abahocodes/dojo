class Solution {
    public long uniquePaths(int m, int n) {
        long[] row = new long[n];
        Arrays.fill(row, 1L);
        for (int r = 1; r < m; r++) {
            for (int c = 1; c < n; c++) row[c] += row[c - 1];
        }
        return row[n - 1];
    }
}
