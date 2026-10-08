class Solution {
    public int swimInWater(int[][] grid) {
        int n = grid.length;
        boolean[][] seen = new boolean[n][n];
        seen[0][0] = true;
        PriorityQueue<int[]> heap = new PriorityQueue<>((a, b) -> Integer.compare(a[0], b[0]));
        heap.add(new int[] {grid[0][0], 0, 0});
        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        int level = 0;
        while (!heap.isEmpty()) {
            int[] top = heap.poll();
            int r = top[1], c = top[2];
            level = Math.max(level, top[0]);
            if (r == n - 1 && c == n - 1) return level;
            for (int[] d : dirs) {
                int nr = r + d[0], nc = c + d[1];
                if (nr >= 0 && nr < n && nc >= 0 && nc < n && !seen[nr][nc]) {
                    seen[nr][nc] = true;
                    heap.add(new int[] {grid[nr][nc], nr, nc});
                }
            }
        }
        return level;
    }
}
