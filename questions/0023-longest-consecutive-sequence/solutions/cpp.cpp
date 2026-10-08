class Solution {
public:
    int longestConsecutiveSequence(vector<int>& nums) {
        unordered_set<long long> values(nums.begin(), nums.end());
        int best = 0;
        for (long long x : values) {
            if (!values.count(x - 1)) {
                int length = 1;
                while (values.count(x + length)) length++;
                best = max(best, length);
            }
        }
        return best;
    }
};
