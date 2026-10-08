class Solution {
public:
    int numOfWays(vector<int>& nums) {
        const long long MOD = 1000000007LL;
        int n = nums.size();
        vector<int> left(n + 1, 0), right(n + 1, 0);
        int root = nums[0];
        for (int i = 1; i < n; i++) {
            int v = nums[i];
            int cur = root;
            while (true) {
                if (v < cur) {
                    if (left[cur] == 0) { left[cur] = v; break; }
                    cur = left[cur];
                } else {
                    if (right[cur] == 0) { right[cur] = v; break; }
                    cur = right[cur];
                }
            }
        }

        auto power = [&](long long base, long long exp) {
            long long result = 1;
            base %= MOD;
            while (exp > 0) {
                if (exp & 1) result = result * base % MOD;
                base = base * base % MOD;
                exp >>= 1;
            }
            return result;
        };
        vector<long long> fact(n + 1, 1), invFact(n + 1, 1);
        for (int i = 1; i <= n; i++) fact[i] = fact[i - 1] * i % MOD;
        invFact[n] = power(fact[n], MOD - 2);
        for (int i = n; i > 0; i--) invFact[i - 1] = invFact[i] * i % MOD;

        vector<int> size(n + 1, 0);
        vector<long long> ways(n + 1, 1);
        for (int i = n - 1; i >= 0; i--) {
            int v = nums[i];
            int l = left[v], r = right[v];
            size[v] = size[l] + size[r] + 1;
            long long interleave = fact[size[l] + size[r]] * invFact[size[l]] % MOD * invFact[size[r]] % MOD;
            ways[v] = interleave * ways[l] % MOD * ways[r] % MOD;
        }
        return (int)((ways[root] - 1 + MOD) % MOD);
    }
};
