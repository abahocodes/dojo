class Solution {
    private static final long MOD = 1_000_000_007L;

    public int numOfWays(int[] nums) {
        int n = nums.length;
        int[] left = new int[n + 1];
        int[] right = new int[n + 1];
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

        long[] fact = new long[n + 1];
        long[] invFact = new long[n + 1];
        fact[0] = 1;
        for (int i = 1; i <= n; i++) fact[i] = fact[i - 1] * i % MOD;
        invFact[n] = power(fact[n], MOD - 2);
        for (int i = n; i > 0; i--) invFact[i - 1] = invFact[i] * i % MOD;

        int[] size = new int[n + 1];
        long[] ways = new long[n + 1];
        Arrays.fill(ways, 1);
        for (int i = n - 1; i >= 0; i--) {
            int v = nums[i];
            int l = left[v], r = right[v];
            size[v] = size[l] + size[r] + 1;
            long interleave = fact[size[l] + size[r]] * invFact[size[l]] % MOD * invFact[size[r]] % MOD;
            ways[v] = interleave * ways[l] % MOD * ways[r] % MOD;
        }
        return (int) ((ways[root] - 1 + MOD) % MOD);
    }

    private long power(long base, long exp) {
        long result = 1;
        base %= MOD;
        while (exp > 0) {
            if ((exp & 1) == 1) result = result * base % MOD;
            base = base * base % MOD;
            exp >>= 1;
        }
        return result;
    }
}
