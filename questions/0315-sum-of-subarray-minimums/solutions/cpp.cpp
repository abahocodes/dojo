class Solution {
public:
    int sumSubarrayMins(vector<int>& arr) {
        const long long MOD = 1000000007LL;
        int n = arr.size();
        vector<int> stack;
        long long total = 0;
        for (int j = 0; j <= n; j++) {
            int cur = j < n ? arr[j] : 0;
            while (!stack.empty() && arr[stack.back()] >= cur) {
                int i = stack.back();
                stack.pop_back();
                int left = stack.empty() ? -1 : stack.back();
                total = (total + (long long)arr[i] * (i - left) % MOD * (j - i)) % MOD;
            }
            stack.push_back(j);
        }
        return (int)total;
    }
};
