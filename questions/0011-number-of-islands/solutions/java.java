class Solution {
    public int numIslands(String[][] grid) {
        if (grid.length == 0) return 0;
        int rows = grid.length, cols = grid[0].length;
        boolean[][] seen = new boolean[rows][cols];
        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        int count = 0;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (!grid[r][c].equals("1") || seen[r][c]) continue;
                count++;
                seen[r][c] = true;
                Deque<int[]> stack = new ArrayDeque<>();
                stack.push(new int[] {r, c});
                while (!stack.isEmpty()) {
                    int[] cell = stack.pop();
                    for (int[] d : dirs) {
                        int ni = cell[0] + d[0], nj = cell[1] + d[1];
                        if (ni >= 0 && ni < rows && nj >= 0 && nj < cols
                                && grid[ni][nj].equals("1") && !seen[ni][nj]) {
                            seen[ni][nj] = true;
                            stack.push(new int[] {ni, nj});
                        }
                    }
                }
            }
        }
        return count;
    }
}
