class Solution {
public:
    int maxAreaOfIsland(vector<vector<int>>& grid) {
        int rows = grid.size();
        int cols = rows > 0 ? grid[0].size() : 0;
        vector<vector<bool>> seen(rows, vector<bool>(cols, false));
        const int dy[4] = {1, -1, 0, 0};
        const int dx[4] = {0, 0, 1, -1};
        int best = 0;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (grid[r][c] != 1 || seen[r][c]) continue;
                // Iterative DFS: count every land cell reachable from (r, c).
                seen[r][c] = true;
                vector<pair<int, int>> stack = {{r, c}};
                int area = 0;
                while (!stack.empty()) {
                    auto [y, x] = stack.back();
                    stack.pop_back();
                    area++;
                    for (int d = 0; d < 4; d++) {
                        int ny = y + dy[d], nx = x + dx[d];
                        if (ny >= 0 && ny < rows && nx >= 0 && nx < cols && grid[ny][nx] == 1 && !seen[ny][nx]) {
                            seen[ny][nx] = true;
                            stack.push_back({ny, nx});
                        }
                    }
                }
                best = max(best, area);
            }
        }
        return best;
    }
};
