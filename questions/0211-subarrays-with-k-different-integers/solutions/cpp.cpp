class Solution {
public:
    int subarraysWithKDistinct(vector<int>& nums, int k) {
        return atMost(nums, k) - atMost(nums, k - 1);
    }

private:
    int atMost(vector<int>& nums, int limit) {
        int n = nums.size();
        vector<int> count(n + 1, 0);
        int distinct = 0, left = 0, total = 0;
        for (int right = 0; right < n; right++) {
            if (count[nums[right]]++ == 0) distinct++;
            while (distinct > limit) {
                if (--count[nums[left]] == 0) distinct--;
                left++;
            }
            total += right - left + 1;
        }
        return total;
    }
};
