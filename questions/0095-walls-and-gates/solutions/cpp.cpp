class Solution {
public:
    vector<vector<int>> wallsAndGates(vector<vector<int>>& rooms) {
        const int EMPTY = INT_MAX;
        int rows = rooms.size(), cols = rooms[0].size();
        vector<vector<int>> dist = rooms;
        queue<pair<int, int>> q;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (dist[r][c] == 0) q.push({r, c});
            }
        }
        const int dirs[4][2] = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        while (!q.empty()) {
            auto [r, c] = q.front();
            q.pop();
            int d = dist[r][c] + 1;
            for (const auto& dir : dirs) {
                int nr = r + dir[0], nc = c + dir[1];
                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && dist[nr][nc] == EMPTY) {
                    dist[nr][nc] = d;
                    q.push({nr, nc});
                }
            }
        }
        return dist;
    }
};
