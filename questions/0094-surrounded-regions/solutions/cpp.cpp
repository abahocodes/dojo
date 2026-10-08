class Solution {
public:
    vector<vector<string>> captureRegions(vector<vector<string>>& board) {
        int rows = board.size(), cols = board[0].size();
        vector<vector<bool>> safe(rows, vector<bool>(cols, false));
        vector<pair<int, int>> stack;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                bool onEdge = r == 0 || c == 0 || r == rows - 1 || c == cols - 1;
                if (onEdge && board[r][c] == "O") {
                    safe[r][c] = true;
                    stack.push_back({r, c});
                }
            }
        }
        const int dirs[4][2] = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        while (!stack.empty()) {
            auto [r, c] = stack.back();
            stack.pop_back();
            for (const auto& d : dirs) {
                int nr = r + d[0], nc = c + d[1];
                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && board[nr][nc] == "O" && !safe[nr][nc]) {
                    safe[nr][nc] = true;
                    stack.push_back({nr, nc});
                }
            }
        }
        vector<vector<string>> out(rows, vector<string>(cols));
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                out[r][c] = safe[r][c] ? "O" : "X";
            }
        }
        return out;
    }
};
