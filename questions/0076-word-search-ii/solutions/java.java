class Solution {
    private static class TrieNode {
        TrieNode[] children = new TrieNode[26];
        int count; // number of non-null children
        String word;
    }

    private static final int[][] DIRS = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
    private char[][] grid;
    private int rows, cols;
    private List<String> found;

    public String[] findWords(String[][] board, String[] words) {
        TrieNode root = new TrieNode();
        for (String word : words) {
            TrieNode node = root;
            for (int i = 0; i < word.length(); i++) {
                int k = word.charAt(i) - 'a';
                if (node.children[k] == null) {
                    node.children[k] = new TrieNode();
                    node.count++;
                }
                node = node.children[k];
            }
            node.word = word;
        }

        rows = board.length;
        cols = board[0].length;
        grid = new char[rows][cols];
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                grid[r][c] = board[r][c].charAt(0);
            }
        }
        found = new ArrayList<>();
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                dfs(r, c, root);
            }
        }
        return found.toArray(new String[0]);
    }

    private void dfs(int r, int c, TrieNode parent) {
        char ch = grid[r][c];
        int k = ch - 'a';
        TrieNode node = parent.children[k];
        if (node == null) return;
        if (node.word != null) {
            found.add(node.word);
            node.word = null;
        }
        grid[r][c] = '#';
        for (int[] d : DIRS) {
            int nr = r + d[0], nc = c + d[1];
            if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] != '#') {
                dfs(nr, nc, node);
            }
        }
        grid[r][c] = ch;
        // prune branches with nothing left to find
        if (node.count == 0 && node.word == null) {
            parent.children[k] = null;
            parent.count--;
        }
    }
}
