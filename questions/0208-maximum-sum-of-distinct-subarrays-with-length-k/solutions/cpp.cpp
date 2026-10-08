class Solution {
public:
    long long maximumSubarraySumDistinct(vector<int>& nums, int k) {
        vector<int> count(100001, 0);
        int dup = 0;
        long long window = 0;
        long long best = 0;
        int n = nums.size();
        for (int i = 0; i < n; i++) {
            int value = nums[i];
            window += value;
            if (++count[value] == 2) dup++;
            if (i >= k) {
                int old = nums[i - k];
                window -= old;
                if (--count[old] == 1) dup--;
            }
            if (i >= k - 1 && dup == 0) best = max(best, window);
        }
        return best;
    }
};
