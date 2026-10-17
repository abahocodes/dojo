class Solution {
public:
    bool mergeTriplets(vector<vector<int>>& triplets, vector<int>& target) {
        // Merge every triplet that never exceeds the target; the rest are poison.
        vector<int> best(3, 0);
        for (auto& t : triplets) {
            if (t[0] <= target[0] && t[1] <= target[1] && t[2] <= target[2]) {
                for (int i = 0; i < 3; i++) best[i] = max(best[i], t[i]);
            }
        }
        return best == target;
    }
};
