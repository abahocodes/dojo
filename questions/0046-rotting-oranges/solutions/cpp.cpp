class Solution {
public:
    int orangesRotting(vector<vector<int>>& grid) {
        int rows = grid.size(), cols = grid[0].size();
        vector<vector<int>> state = grid;  // don't mutate the caller's grid
        vector<pair<int, int>> frontier;
        int fresh = 0;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (state[r][c] == 2) frontier.push_back({r, c});
                else if (state[r][c] == 1) fresh++;
            }
        }

        const int dirs[4][2] = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        int minutes = 0;
        while (!frontier.empty() && fresh > 0) {
            minutes++;
            vector<pair<int, int>> next;
            for (auto [r, c] : frontier) {
                for (auto& d : dirs) {
                    int nr = r + d[0], nc = c + d[1];
                    if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && state[nr][nc] == 1) {
                        state[nr][nc] = 2;
                        fresh--;
                        next.push_back({nr, nc});
                    }
                }
            }
            frontier = move(next);
        }
        return fresh == 0 ? minutes : -1;
    }
};
