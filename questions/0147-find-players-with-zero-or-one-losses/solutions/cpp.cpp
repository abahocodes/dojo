class Solution {
public:
    vector<vector<int>> findWinners(vector<vector<int>>& matches) {
        int maxId = 0;
        for (auto& m : matches) maxId = max({maxId, m[0], m[1]});
        vector<int> losses(maxId + 1, -1); // -1: never played
        for (auto& m : matches) {
            if (losses[m[0]] < 0) losses[m[0]] = 0;
            losses[m[1]] = max(losses[m[1]], 0) + 1;
        }
        vector<vector<int>> result(2);
        for (int p = 1; p <= maxId; p++) {
            if (losses[p] == 0) result[0].push_back(p);
            else if (losses[p] == 1) result[1].push_back(p);
        }
        return result;
    }
};
