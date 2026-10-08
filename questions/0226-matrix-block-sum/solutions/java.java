class Solution {
    public int[][] matrixBlockSum(int[][] mat, int k) {
        int m = mat.length, n = mat[0].length;
        int[][] pre = new int[m + 1][n + 1];
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                pre[i + 1][j + 1] = mat[i][j] + pre[i][j + 1] + pre[i + 1][j] - pre[i][j];
            }
        }
        int[][] result = new int[m][n];
        for (int i = 0; i < m; i++) {
            int r1 = Math.max(0, i - k), r2 = Math.min(m, i + k + 1);
            for (int j = 0; j < n; j++) {
                int c1 = Math.max(0, j - k), c2 = Math.min(n, j + k + 1);
                result[i][j] = pre[r2][c2] - pre[r1][c2] - pre[r2][c1] + pre[r1][c1];
            }
        }
        return result;
    }
}
