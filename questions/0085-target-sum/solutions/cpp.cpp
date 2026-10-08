class Solution {
public:
    int findTargetSumWays(vector<int>& nums, int target) {
        int total = accumulate(nums.begin(), nums.end(), 0);
        if (abs(target) > total || (total + target) % 2 != 0) return 0;
        int goal = (total + target) / 2;
        vector<int> ways(goal + 1, 0);
        ways[0] = 1;
        for (int x : nums) {
            for (int s = goal; s >= x; s--) {
                ways[s] += ways[s - x];
            }
        }
        return ways[goal];
    }
};
