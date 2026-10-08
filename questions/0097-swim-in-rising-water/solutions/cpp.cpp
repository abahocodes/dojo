class Solution {
public:
    int swimInWater(vector<vector<int>>& grid) {
        int n = grid.size();
        vector<vector<bool>> seen(n, vector<bool>(n, false));
        seen[0][0] = true;
        priority_queue<tuple<int, int, int>, vector<tuple<int, int, int>>, greater<>> heap;
        heap.push({grid[0][0], 0, 0});
        const int dirs[4][2] = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        int level = 0;
        while (!heap.empty()) {
            auto [h, r, c] = heap.top();
            heap.pop();
            level = max(level, h);
            if (r == n - 1 && c == n - 1) return level;
            for (const auto& d : dirs) {
                int nr = r + d[0], nc = c + d[1];
                if (nr >= 0 && nr < n && nc >= 0 && nc < n && !seen[nr][nc]) {
                    seen[nr][nc] = true;
                    heap.push({grid[nr][nc], nr, nc});
                }
            }
        }
        return level;
    }
};
