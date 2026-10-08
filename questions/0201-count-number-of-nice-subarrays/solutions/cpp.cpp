class Solution {
public:
    int numberOfNiceSubarrays(vector<int>& nums, int k) {
        vector<int> seen(nums.size() + 1, 0);
        seen[0] = 1;
        int odds = 0, total = 0;
        for (int x : nums) {
            odds += x & 1;
            if (odds >= k) total += seen[odds - k];
            seen[odds]++;
        }
        return total;
    }
};
