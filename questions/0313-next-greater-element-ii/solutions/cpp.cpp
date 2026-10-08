class Solution {
public:
    vector<int> nextGreaterCircular(vector<int>& nums) {
        int n = nums.size();
        vector<int> result(n, -1);
        vector<int> stack;
        for (int j = 0; j < 2 * n; j++) {
            int x = nums[j % n];
            while (!stack.empty() && nums[stack.back()] < x) {
                result[stack.back()] = x;
                stack.pop_back();
            }
            if (j < n) stack.push_back(j);
        }
        return result;
    }
};
