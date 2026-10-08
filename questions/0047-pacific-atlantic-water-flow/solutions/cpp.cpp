class Solution {
public:
    vector<vector<int>> pacificAtlantic(vector<vector<int>>& heights) {
        int rows = heights.size(), cols = heights[0].size();
        vector<pair<int, int>> pacificStarts, atlanticStarts;
        for (int c = 0; c < cols; c++) {
            pacificStarts.push_back({0, c});
            atlanticStarts.push_back({rows - 1, c});
        }
        for (int r = 0; r < rows; r++) {
            pacificStarts.push_back({r, 0});
            atlanticStarts.push_back({r, cols - 1});
        }
        vector<vector<bool>> pacific = reachable(heights, pacificStarts);
        vector<vector<bool>> atlantic = reachable(heights, atlanticStarts);
        vector<vector<int>> result;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (pacific[r][c] && atlantic[r][c]) result.push_back({r, c});
            }
        }
        return result;
    }

private:
    // Walk uphill from the ocean: water can flow from each reached cell
    // down to the ocean. Iterative DFS keeps large grids off the call stack.
    vector<vector<bool>> reachable(const vector<vector<int>>& heights,
                                   const vector<pair<int, int>>& starts) {
        int rows = heights.size(), cols = heights[0].size();
        vector<vector<bool>> seen(rows, vector<bool>(cols, false));
        vector<pair<int, int>> stack;
        for (auto [r, c] : starts) {
            if (!seen[r][c]) {
                seen[r][c] = true;
                stack.push_back({r, c});
            }
        }
        const int dirs[4][2] = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        while (!stack.empty()) {
            auto [r, c] = stack.back();
            stack.pop_back();
            for (auto& d : dirs) {
                int nr = r + d[0], nc = c + d[1];
                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && !seen[nr][nc]
                        && heights[nr][nc] >= heights[r][c]) {
                    seen[nr][nc] = true;
                    stack.push_back({nr, nc});
                }
            }
        }
        return seen;
    }
};
