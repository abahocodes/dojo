class Solution {
public:
    int maximumScore(vector<int>& nums, int k) {
        int n = nums.size();
        int i = k, j = k;
        int low = nums[k];
        int best = low;
        while (i > 0 || j < n - 1) {
            if (i == 0 || (j < n - 1 && nums[j + 1] > nums[i - 1])) {
                j++;
                low = min(low, nums[j]);
            } else {
                i--;
                low = min(low, nums[i]);
            }
            best = max(best, low * (j - i + 1));
        }
        return best;
    }
};
