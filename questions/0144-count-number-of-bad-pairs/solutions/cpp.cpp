class Solution {
public:
    long long countBadPairs(vector<int>& nums) {
        unordered_map<int, int> seen;
        long long good = 0;
        for (int j = 0; j < (int)nums.size(); j++) {
            good += seen[nums[j] - j]++;
        }
        long long n = nums.size();
        return n * (n - 1) / 2 - good;
    }
};
