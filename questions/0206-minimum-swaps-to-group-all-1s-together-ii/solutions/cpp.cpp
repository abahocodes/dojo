class Solution {
public:
    int minSwapsCircular(vector<int>& nums) {
        int n = nums.size();
        int ones = 0;
        for (int v : nums) ones += v;
        if (ones == 0) return 0;
        int window = 0;
        for (int i = 0; i < ones; i++) window += nums[i];
        int best = window;
        for (int i = ones; i < ones + n - 1; i++) {
            window += nums[i % n] - nums[i - ones];
            best = max(best, window);
        }
        return ones - best;
    }
};
