class Solution {
    private char[][] grid;
    private char[] w;

    public boolean exist(String[][] board, String word) {
        int rows = board.length, cols = board[0].length;
        grid = new char[rows][cols];
        int[] onBoard = new int[128];
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                grid[r][c] = board[r][c].charAt(0);
                onBoard[grid[r][c]]++;
            }
        }
        int[] needed = new int[128];
        for (char ch : word.toCharArray()) {
            if (++needed[ch] > onBoard[ch]) return false;
        }
        // a path read backwards is still a path; start from the rarer end to prune sooner
        w = word.toCharArray();
        if (onBoard[w[0]] > onBoard[w[w.length - 1]]) {
            w = new StringBuilder(word).reverse().toString().toCharArray();
        }
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (dfs(r, c, 0)) return true;
            }
        }
        return false;
    }

    private boolean dfs(int r, int c, int i) {
        if (r < 0 || r >= grid.length || c < 0 || c >= grid[0].length || grid[r][c] != w[i]) return false;
        if (i == w.length - 1) return true;
        grid[r][c] = '#'; // mark as used on the current path
        boolean found = dfs(r + 1, c, i + 1) || dfs(r - 1, c, i + 1)
                || dfs(r, c + 1, i + 1) || dfs(r, c - 1, i + 1);
        grid[r][c] = w[i];
        return found;
    }
}
