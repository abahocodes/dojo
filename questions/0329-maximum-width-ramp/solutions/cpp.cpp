class Solution {
public:
    int maxWidthRamp(vector<int>& nums) {
        vector<int> stack;
        for (int i = 0; i < (int) nums.size(); i++) {
            if (stack.empty() || nums[i] < nums[stack.back()]) stack.push_back(i);
        }
        int best = 0;
        for (int j = (int) nums.size() - 1; j >= 0; j--) {
            while (!stack.empty() && nums[stack.back()] <= nums[j]) {
                best = max(best, j - stack.back());
                stack.pop_back();
            }
        }
        return best;
    }
};
