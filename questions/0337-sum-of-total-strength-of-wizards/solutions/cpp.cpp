class Solution {
public:
    int totalStrength(vector<int>& strength) {
        const long long MOD = 1000000007LL;
        int n = strength.size();
        vector<int> left(n, -1), right(n, n), stk;
        for (int i = 0; i < n; i++) {
            while (!stk.empty() && strength[stk.back()] >= strength[i]) {
                right[stk.back()] = i;  // first smaller-or-equal to the right
                stk.pop_back();
            }
            left[i] = stk.empty() ? -1 : stk.back();  // first strictly smaller to the left
            stk.push_back(i);
        }
        // pp[k] = P[0] + ... + P[k-1], where P[k] = strength[0] + ... + strength[k-1]
        vector<long long> pp(n + 2, 0);
        long long p = 0;
        for (int k = 0; k <= n; k++) {
            pp[k + 1] = (pp[k] + p) % MOD;
            if (k < n) p = (p + strength[k]) % MOD;
        }
        long long total = 0;
        for (int i = 0; i < n; i++) {
            long long l = left[i], r = right[i];
            long long plus = (i - l) * ((pp[r + 1] - pp[i + 1] + MOD) % MOD) % MOD;
            long long minus = (r - i) * ((pp[i + 1] - pp[l + 1] + MOD) % MOD) % MOD;
            long long span = (plus - minus + MOD) % MOD;
            total = (total + (long long)strength[i] % MOD * span) % MOD;
        }
        return (int)total;
    }
};
