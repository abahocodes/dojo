class Solution {
    public int totalStrength(int[] strength) {
        final long MOD = 1_000_000_007L;
        int n = strength.length;
        int[] left = new int[n];
        int[] right = new int[n];
        int[] stack = new int[n];
        int top = 0;
        for (int i = 0; i < n; i++) {
            while (top > 0 && strength[stack[top - 1]] >= strength[i]) {
                right[stack[--top]] = i; // first smaller-or-equal to the right
            }
            left[i] = top > 0 ? stack[top - 1] : -1; // first strictly smaller to the left
            stack[top++] = i;
        }
        while (top > 0) right[stack[--top]] = n;
        // pp[k] = P[0] + ... + P[k-1], where P[k] = strength[0] + ... + strength[k-1]
        long[] pp = new long[n + 2];
        long p = 0;
        for (int k = 0; k <= n; k++) {
            pp[k + 1] = (pp[k] + p) % MOD;
            if (k < n) p = (p + strength[k]) % MOD;
        }
        long total = 0;
        for (int i = 0; i < n; i++) {
            int l = left[i];
            int r = right[i];
            long plus = (i - l) * ((pp[r + 1] - pp[i + 1] + MOD) % MOD) % MOD;
            long minus = (r - i) * ((pp[i + 1] - pp[l + 1] + MOD) % MOD) % MOD;
            long span = (plus - minus + MOD) % MOD;
            total = (total + strength[i] % MOD * span) % MOD;
        }
        return (int) total;
    }
}
