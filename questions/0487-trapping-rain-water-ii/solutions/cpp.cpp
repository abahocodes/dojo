class Solution {
public:
    int trapRainWater2d(vector<vector<int>>& heightMap) {
        int m = heightMap.size();
        int n = heightMap[0].size();
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        // Frontier cells (level, row, col), lowest level first.
        priority_queue<array<int, 3>, vector<array<int, 3>>, greater<array<int, 3>>> heap;
        for (int r = 0; r < m; r++) {
            for (int c = 0; c < n; c++) {
                if (r == 0 || c == 0 || r == m - 1 || c == n - 1) {
                    heap.push({heightMap[r][c], r, c});
                    visited[r][c] = true;
                }
            }
        }

        const int dr[] = {1, -1, 0, 0};
        const int dc[] = {0, 0, 1, -1};
        int total = 0;
        while (!heap.empty()) {
            auto [level, r, c] = heap.top();
            heap.pop();
            for (int d = 0; d < 4; d++) {
                int nr = r + dr[d];
                int nc = c + dc[d];
                if (nr < 0 || nc < 0 || nr >= m || nc >= n || visited[nr][nc]) continue;
                visited[nr][nc] = true;
                int h = heightMap[nr][nc];
                if (h < level) total += level - h;
                heap.push({max(h, level), nr, nc});
            }
        }
        return total;
    }
};
