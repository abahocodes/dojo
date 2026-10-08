class Solution {
public:
    bool find132Pattern(vector<int>& nums) {
        int third = INT_MIN; // every value is > -2^31, so this means "none yet"
        vector<int> stack;
        for (int i = (int)nums.size() - 1; i >= 0; i--) {
            int x = nums[i];
            if (x < third) return true;
            while (!stack.empty() && stack.back() < x) {
                third = stack.back();
                stack.pop_back();
            }
            stack.push_back(x);
        }
        return false;
    }
};
