class Solution {
public:
    int findMinArrowShots(vector<vector<int>>& points) {
        // Greedy: shoot each arrow at the right end of the balloon that ends first.
        vector<vector<int>> ordered = points;
        sort(ordered.begin(), ordered.end(),
             [](const vector<int>& a, const vector<int>& b) { return a[1] < b[1]; });
        int arrows = 1;
        int pos = ordered[0][1];
        for (auto& p : ordered) {
            if (p[0] > pos) {
                arrows++;
                pos = p[1];
            }
        }
        return arrows;
    }
};
