class Solution {
    public int[][] diagonalSort(int[][] mat) {
        int m = mat.length, n = mat[0].length;
        int[][] res = new int[m][];
        for (int i = 0; i < m; i++) res[i] = mat[i].clone();
        for (int i = 0; i < m; i++) sortFrom(res, i, 0);
        for (int j = 1; j < n; j++) sortFrom(res, 0, j);
        return res;
    }

    private void sortFrom(int[][] res, int si, int sj) {
        int length = Math.min(res.length - si, res[0].length - sj);
        int[] values = new int[length];
        for (int k = 0; k < length; k++) values[k] = res[si + k][sj + k];
        Arrays.sort(values);
        for (int k = 0; k < length; k++) res[si + k][sj + k] = values[k];
    }
}
