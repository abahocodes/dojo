class Solution {
public:
    long long countSubarraysMaxK(vector<int>& nums, int k) {
        int m = *max_element(nums.begin(), nums.end());
        int count = 0;
        int left = 0;
        long long total = 0;
        for (int v : nums) {
            if (v == m) count++;
            while (count >= k) {
                if (nums[left] == m) count--;
                left++;
            }
            total += left;
        }
        return total;
    }
};
