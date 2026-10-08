class Solution {
public:
    double findMaxAverage(vector<int>& nums, int k) {
        int window = 0;
        for (int i = 0; i < k; i++) window += nums[i];
        int best = window;
        for (int i = k; i < (int)nums.size(); i++) {
            window += nums[i] - nums[i - k];
            best = max(best, window);
        }
        return (double)best / k;
    }
};
