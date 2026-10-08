class Solution {
public:
    int minOperationsReduceX(vector<int>& nums, int x) {
        long long total = 0;
        for (int v : nums) total += v;
        long long target = total - x;
        if (target < 0) return -1;
        int n = nums.size();
        int best = -1;
        long long window = 0;
        int left = 0;
        for (int right = 0; right < n; right++) {
            window += nums[right];
            while (window > target) window -= nums[left++];
            if (window == target) best = max(best, right - left + 1);
        }
        return best == -1 ? -1 : n - best;
    }
};
