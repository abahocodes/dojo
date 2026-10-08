class Solution {
public:
    int findUnsortedSubarray(vector<int>& nums) {
        int n = nums.size();
        int end = -1;
        int runningMax = nums[0];
        for (int i = 1; i < n; i++) {
            if (nums[i] < runningMax) end = i;
            else runningMax = nums[i];
        }
        if (end == -1) return 0;
        int start = n;
        int runningMin = nums[n - 1];
        for (int i = n - 2; i >= 0; i--) {
            if (nums[i] > runningMin) start = i;
            else runningMin = nums[i];
        }
        return end - start + 1;
    }
};
