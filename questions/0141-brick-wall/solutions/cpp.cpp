class Solution {
public:
    int leastBricks(vector<vector<int>>& wall) {
        unordered_map<int, int> seams;
        int best = 0;
        for (auto& row : wall) {
            int pos = 0;
            for (size_t i = 0; i + 1 < row.size(); i++) {
                pos += row[i];
                best = max(best, ++seams[pos]);
            }
        }
        return (int)wall.size() - best;
    }
};
