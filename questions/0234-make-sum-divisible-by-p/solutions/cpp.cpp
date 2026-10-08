class Solution {
public:
    int minSubarrayRemove(vector<int>& nums, int p) {
        long long need = 0;
        for (int x : nums) need = (need + x) % p;
        if (need == 0) return 0;
        unordered_map<long long, int> latest;
        latest.reserve(nums.size() * 2 + 1);
        latest[0] = -1;
        long long cur = 0;
        int n = nums.size();
        int best = n;
        for (int j = 0; j < n; j++) {
            cur = (cur + nums[j]) % p;
            long long want = (cur - need + p) % p;
            auto it = latest.find(want);
            if (it != latest.end()) best = min(best, j - it->second);
            latest[cur] = j;
        }
        return best < n ? best : -1;
    }
};
