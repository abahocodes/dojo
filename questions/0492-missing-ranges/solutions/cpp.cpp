class Solution {
public:
    vector<vector<int>> findMissingRanges(vector<int>& nums, int lower, int upper) {
        vector<vector<int>> ranges;
        long long prev = (long long)lower - 1; // last value known to be present
        for (size_t i = 0; i <= nums.size(); i++) {
            long long x = i < nums.size() ? nums[i] : (long long)upper + 1;
            if (x - prev >= 2) ranges.push_back({(int)(prev + 1), (int)(x - 1)});
            prev = x;
        }
        return ranges;
    }
};
