class Solution {
public:
    int longestSquareStreak(vector<int>& nums) {
        int mx = *max_element(nums.begin(), nums.end());
        vector<bool> present(mx + 1, false);
        for (int x : nums) present[x] = true;
        int best = -1;
        for (int x = 2; x <= mx; x++) {
            if (!present[x]) continue;
            int length = 1;
            long long cur = x;
            while (cur * cur <= mx && present[cur * cur]) {
                cur *= cur;
                length++;
            }
            if (length >= 2 && length > best) best = length;
        }
        return best;
    }
};
