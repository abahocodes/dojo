class Solution {
public:
    int minimumDifference(vector<int>& nums, int k) {
        vector<int> s(nums);
        sort(s.begin(), s.end());
        int best = INT_MAX;
        for (int i = 0; i + k - 1 < (int)s.size(); i++) {
            best = min(best, s[i + k - 1] - s[i]);
        }
        return best;
    }
};
