class Solution {
    public int maxCoins(int[] nums) {
        int n = nums.length + 2;
        int[] v = new int[n];
        v[0] = 1;
        v[n - 1] = 1;
        System.arraycopy(nums, 0, v, 1, nums.length);
        // best[i][j] is the most coins from bursting everything strictly between i and j
        int[][] best = new int[n][n];
        for (int length = 2; length < n; length++) {
            for (int i = 0; i + length < n; i++) {
                int j = i + length;
                int edge = v[i] * v[j];
                int top = 0;
                for (int k = i + 1; k < j; k++) {
                    top = Math.max(top, best[i][k] + edge * v[k] + best[k][j]);
                }
                best[i][j] = top;
            }
        }
        return best[0][n - 1];
    }
}
