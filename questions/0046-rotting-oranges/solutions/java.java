class Solution {
    public int orangesRotting(int[][] grid) {
        int rows = grid.length, cols = grid[0].length;
        int[][] state = new int[rows][];
        for (int r = 0; r < rows; r++) state[r] = grid[r].clone(); // don't mutate the caller's grid
        List<int[]> frontier = new ArrayList<>();
        int fresh = 0;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (state[r][c] == 2) frontier.add(new int[]{r, c});
                else if (state[r][c] == 1) fresh++;
            }
        }

        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        int minutes = 0;
        while (!frontier.isEmpty() && fresh > 0) {
            minutes++;
            List<int[]> next = new ArrayList<>();
            for (int[] cell : frontier) {
                for (int[] d : dirs) {
                    int nr = cell[0] + d[0], nc = cell[1] + d[1];
                    if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && state[nr][nc] == 1) {
                        state[nr][nc] = 2;
                        fresh--;
                        next.add(new int[]{nr, nc});
                    }
                }
            }
            frontier = next;
        }
        return fresh == 0 ? minutes : -1;
    }
}
