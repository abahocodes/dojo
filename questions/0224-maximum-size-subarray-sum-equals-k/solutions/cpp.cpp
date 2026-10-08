class Solution {
public:
    int maxSubArrayLen(vector<int>& nums, int k) {
        unordered_map<long long, int> first;
        first[0] = -1;
        long long prefix = 0;
        int best = 0;
        for (int i = 0; i < (int)nums.size(); i++) {
            prefix += nums[i];
            auto it = first.find(prefix - k);
            if (it != first.end()) best = max(best, i - it->second);
            first.emplace(prefix, i);  // keeps the earliest index
        }
        return best;
    }
};
