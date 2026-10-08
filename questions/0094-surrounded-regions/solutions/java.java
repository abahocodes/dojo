class Solution {
    public String[][] captureRegions(String[][] board) {
        int rows = board.length, cols = board[0].length;
        boolean[][] safe = new boolean[rows][cols];
        Deque<int[]> stack = new ArrayDeque<>();
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                boolean onEdge = r == 0 || c == 0 || r == rows - 1 || c == cols - 1;
                if (onEdge && board[r][c].equals("O")) {
                    safe[r][c] = true;
                    stack.push(new int[] {r, c});
                }
            }
        }
        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        while (!stack.isEmpty()) {
            int[] cell = stack.pop();
            for (int[] d : dirs) {
                int nr = cell[0] + d[0], nc = cell[1] + d[1];
                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols
                        && board[nr][nc].equals("O") && !safe[nr][nc]) {
                    safe[nr][nc] = true;
                    stack.push(new int[] {nr, nc});
                }
            }
        }
        String[][] out = new String[rows][cols];
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                out[r][c] = safe[r][c] ? "O" : "X";
            }
        }
        return out;
    }
}
