class Solution {
    public int trapRainWater2d(int[][] heightMap) {
        int m = heightMap.length;
        int n = heightMap[0].length;
        boolean[][] visited = new boolean[m][n];
        // Frontier cells {level, row, col}, lowest level first.
        PriorityQueue<int[]> heap = new PriorityQueue<>((a, b) -> Integer.compare(a[0], b[0]));
        for (int r = 0; r < m; r++) {
            for (int c = 0; c < n; c++) {
                if (r == 0 || c == 0 || r == m - 1 || c == n - 1) {
                    heap.offer(new int[] {heightMap[r][c], r, c});
                    visited[r][c] = true;
                }
            }
        }

        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        int total = 0;
        while (!heap.isEmpty()) {
            int[] cell = heap.poll();
            int level = cell[0];
            for (int[] d : dirs) {
                int nr = cell[1] + d[0];
                int nc = cell[2] + d[1];
                if (nr < 0 || nc < 0 || nr >= m || nc >= n || visited[nr][nc]) continue;
                visited[nr][nc] = true;
                int h = heightMap[nr][nc];
                if (h < level) total += level - h;
                heap.offer(new int[] {Math.max(h, level), nr, nc});
            }
        }
        return total;
    }
}
