class Solution {
    public int equalPairs(int[][] grid) {
        int n = grid.length;
        Map<String, Integer> rows = new HashMap<>();
        for (int[] row : grid) {
            rows.merge(Arrays.toString(row), 1, Integer::sum);
        }
        int pairs = 0;
        int[] col = new int[n];
        for (int c = 0; c < n; c++) {
            for (int r = 0; r < n; r++) col[r] = grid[r][c];
            pairs += rows.getOrDefault(Arrays.toString(col), 0);
        }
        return pairs;
    }
}
