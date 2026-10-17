class Solution {
public:
    vector<vector<string>> solveNQueens(int n) {
        this->n = n;
        cols.assign(n, false);
        diag.assign(2 * n - 1, false);
        anti.assign(2 * n - 1, false);
        board.assign(n, string(n, '.'));
        boards.clear();
        place(0);
        return boards;
    }

private:
    int n = 0;
    vector<bool> cols, diag, anti; // diag indexed by r - c + n - 1, anti by r + c
    vector<string> board;
    vector<vector<string>> boards;

    void place(int r) {
        if (r == n) {
            boards.push_back(board);
            return;
        }
        for (int c = 0; c < n; c++) {
            if (cols[c] || diag[r - c + n - 1] || anti[r + c]) continue;
            board[r][c] = 'Q';
            cols[c] = diag[r - c + n - 1] = anti[r + c] = true;
            place(r + 1);
            cols[c] = diag[r - c + n - 1] = anti[r + c] = false;
            board[r][c] = '.';
        }
    }
};
