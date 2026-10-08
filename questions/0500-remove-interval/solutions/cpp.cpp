class Solution {
public:
    vector<vector<int>> removeInterval(vector<vector<int>>& intervals, vector<int>& toBeRemoved) {
        int cutLo = toBeRemoved[0], cutHi = toBeRemoved[1];
        vector<vector<int>> result;
        for (auto& iv : intervals) {
            int a = iv[0], b = iv[1];
            if (b <= cutLo || a >= cutHi) {
                result.push_back({a, b}); // untouched
                continue;
            }
            if (a < cutLo) result.push_back({a, cutLo}); // piece left of the cut
            if (b > cutHi) result.push_back({cutHi, b}); // piece right of the cut
        }
        return result;
    }
};
