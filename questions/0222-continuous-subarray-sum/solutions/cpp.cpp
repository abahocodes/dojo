class Solution {
public:
    bool checkSubarraySum(vector<int>& nums, int k) {
        unordered_map<long long, int> first;
        first[0] = -1;
        long long rem = 0;
        for (int i = 0; i < (int)nums.size(); i++) {
            rem = (rem + nums[i]) % k;
            auto it = first.find(rem);
            if (it != first.end()) {
                if (i - it->second >= 2) return true;
            } else {
                first[rem] = i;
            }
        }
        return false;
    }
};
