class Solution {
    private static final int EMPTY = Integer.MAX_VALUE;

    public int[][] wallsAndGates(int[][] rooms) {
        int rows = rooms.length, cols = rooms[0].length;
        int[][] dist = new int[rows][];
        for (int r = 0; r < rows; r++) dist[r] = rooms[r].clone();
        ArrayDeque<int[]> queue = new ArrayDeque<>();
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (dist[r][c] == 0) queue.add(new int[] {r, c});
            }
        }
        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        while (!queue.isEmpty()) {
            int[] cell = queue.poll();
            int d = dist[cell[0]][cell[1]] + 1;
            for (int[] dir : dirs) {
                int nr = cell[0] + dir[0], nc = cell[1] + dir[1];
                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && dist[nr][nc] == EMPTY) {
                    dist[nr][nc] = d;
                    queue.add(new int[] {nr, nc});
                }
            }
        }
        return dist;
    }
}
