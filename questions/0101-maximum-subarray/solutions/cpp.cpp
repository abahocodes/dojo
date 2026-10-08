class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int current = nums[0], best = nums[0];
        for (size_t i = 1; i < nums.size(); i++) {
            int x = nums[i];
            current = max(x, current + x);
            best = max(best, current);
        }
        return best;
    }
};
