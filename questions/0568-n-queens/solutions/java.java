class Solution {
    private int n;
    private boolean[] cols, diag, anti; // diag indexed by r - c + n - 1, anti by r + c
    private int[] queenCol;             // queenCol[r] = column of the queen in row r
    private List<String[]> boards;

    public String[][] solveNQueens(int n) {
        this.n = n;
        cols = new boolean[n];
        diag = new boolean[2 * n - 1];
        anti = new boolean[2 * n - 1];
        queenCol = new int[n];
        boards = new ArrayList<>();
        place(0);
        return boards.toArray(new String[0][]);
    }

    private void place(int r) {
        if (r == n) {
            String[] board = new String[n];
            for (int i = 0; i < n; i++) {
                char[] row = new char[n];
                Arrays.fill(row, '.');
                row[queenCol[i]] = 'Q';
                board[i] = new String(row);
            }
            boards.add(board);
            return;
        }
        for (int c = 0; c < n; c++) {
            if (cols[c] || diag[r - c + n - 1] || anti[r + c]) continue;
            queenCol[r] = c;
            cols[c] = diag[r - c + n - 1] = anti[r + c] = true;
            place(r + 1);
            cols[c] = diag[r - c + n - 1] = anti[r + c] = false;
        }
    }
}
