class Solution {
public:
    int minOperationsContinuous(vector<int>& nums) {
        int n = nums.size();
        vector<int> u(nums);
        sort(u.begin(), u.end());
        u.erase(unique(u.begin(), u.end()), u.end());
        int best = 0;
        int j = 0;
        int k = u.size();
        for (int i = 0; i < k; i++) {
            long long limit = (long long)u[i] + n - 1;
            while (j < k && u[j] <= limit) j++;
            best = max(best, j - i);
        }
        return n - best;
    }
};
