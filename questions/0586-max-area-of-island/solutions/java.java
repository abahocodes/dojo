class Solution {
    public int maxAreaOfIsland(int[][] grid) {
        int rows = grid.length;
        int cols = rows > 0 ? grid[0].length : 0;
        boolean[][] seen = new boolean[rows][cols];
        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        int best = 0;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (grid[r][c] != 1 || seen[r][c]) continue;
                // Iterative DFS: count every land cell reachable from (r, c).
                seen[r][c] = true;
                Deque<int[]> stack = new ArrayDeque<>();
                stack.push(new int[] {r, c});
                int area = 0;
                while (!stack.isEmpty()) {
                    int[] cell = stack.pop();
                    area++;
                    for (int[] d : dirs) {
                        int ny = cell[0] + d[0];
                        int nx = cell[1] + d[1];
                        if (ny >= 0 && ny < rows && nx >= 0 && nx < cols && grid[ny][nx] == 1 && !seen[ny][nx]) {
                            seen[ny][nx] = true;
                            stack.push(new int[] {ny, nx});
                        }
                    }
                }
                best = Math.max(best, area);
            }
        }
        return best;
    }
}
