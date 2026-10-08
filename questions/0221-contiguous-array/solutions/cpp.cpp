class Solution {
public:
    int findMaxLength(vector<int>& nums) {
        int n = nums.size();
        // balance ranges over [-n, n]; store first index at balance + n
        vector<int> first(2 * n + 1, -2);
        first[n] = -1;
        int balance = 0, best = 0;
        for (int i = 0; i < n; i++) {
            balance += nums[i] == 1 ? 1 : -1;
            int slot = balance + n;
            if (first[slot] != -2) best = max(best, i - first[slot]);
            else first[slot] = i;
        }
        return best;
    }
};
