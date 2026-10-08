class Solution {
    public int[] regionSums(int[][] matrix, int[][] queries) {
        int m = matrix.length, n = matrix[0].length;
        // pre[i][j] = sum of matrix[0..i-1][0..j-1]
        int[][] pre = new int[m + 1][n + 1];
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                pre[i + 1][j + 1] = matrix[i][j] + pre[i][j + 1] + pre[i + 1][j] - pre[i][j];
            }
        }
        int[] result = new int[queries.length];
        for (int q = 0; q < queries.length; q++) {
            int r1 = queries[q][0], c1 = queries[q][1], r2 = queries[q][2], c2 = queries[q][3];
            result[q] = pre[r2 + 1][c2 + 1] - pre[r1][c2 + 1] - pre[r2 + 1][c1] + pre[r1][c1];
        }
        return result;
    }
}
