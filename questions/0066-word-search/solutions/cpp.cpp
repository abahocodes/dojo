class Solution {
public:
    bool exist(vector<vector<string>>& board, string& word) {
        int rows = board.size(), cols = board[0].size();
        grid.assign(rows, string(cols, ' '));
        array<int, 128> onBoard{};
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                grid[r][c] = board[r][c][0];
                onBoard[(unsigned char)grid[r][c]]++;
            }
        }
        array<int, 128> needed{};
        for (char ch : word) {
            if (++needed[(unsigned char)ch] > onBoard[(unsigned char)ch]) return false;
        }
        // a path read backwards is still a path; start from the rarer end to prune sooner
        w = word;
        if (onBoard[(unsigned char)w.front()] > onBoard[(unsigned char)w.back()]) reverse(w.begin(), w.end());
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (dfs(r, c, 0)) return true;
            }
        }
        return false;
    }

private:
    vector<string> grid;
    string w;

    bool dfs(int r, int c, size_t i) {
        if (r < 0 || r >= (int)grid.size() || c < 0 || c >= (int)grid[0].size() || grid[r][c] != w[i]) return false;
        if (i == w.size() - 1) return true;
        grid[r][c] = '#'; // mark as used on the current path
        bool found = dfs(r + 1, c, i + 1) || dfs(r - 1, c, i + 1)
                  || dfs(r, c + 1, i + 1) || dfs(r, c - 1, i + 1);
        grid[r][c] = w[i];
        return found;
    }
};
