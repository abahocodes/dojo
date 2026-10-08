class Solution {
public:
    int numSubseq(vector<int>& nums, int target) {
        const int MOD = 1000000007;
        vector<int> a(nums);
        sort(a.begin(), a.end());
        int n = a.size();
        vector<int> pow2(n, 1);
        for (int i = 1; i < n; i++) pow2[i] = (int)(pow2[i - 1] * 2LL % MOD);
        long long total = 0;
        int lo = 0, hi = n - 1;
        while (lo <= hi) {
            if (a[lo] + a[hi] <= target) {
                total = (total + pow2[hi - lo]) % MOD;
                lo++;
            } else {
                hi--;
            }
        }
        return (int)total;
    }
};
