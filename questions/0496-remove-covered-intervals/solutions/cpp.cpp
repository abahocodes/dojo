class Solution {
public:
    int removeCoveredIntervals(vector<vector<int>>& intervals) {
        // Start ascending; for equal starts the longer interval comes first.
        vector<vector<int>> ordered = intervals;
        sort(ordered.begin(), ordered.end(), [](const vector<int>& a, const vector<int>& b) {
            return a[0] != b[0] ? a[0] < b[0] : a[1] > b[1];
        });
        int remaining = 0;
        int maxEnd = -1;
        for (auto& iv : ordered) {
            if (iv[1] > maxEnd) {
                remaining++;
                maxEnd = iv[1];
            }
        }
        return remaining;
    }
};
