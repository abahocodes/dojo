class Solution {
    private static final long LIMIT = 2_000_000_000L;

    private long gcd(long p, long q) {
        while (q != 0) {
            long t = p % q;
            p = q;
            q = t;
        }
        return p;
    }

    private long lcm(long p, long q) {
        long r = p / gcd(p, q);
        if (r > (LIMIT + 1) / q) return LIMIT + 1;
        return Math.min(r * q, LIMIT + 1);
    }

    public long nthUglyNumber(int n, int a, int b, int c) {
        long ab = lcm(a, b), ac = lcm(a, c), bc = lcm(b, c);
        long abc = lcm(ab, c);
        long lo = 1, hi = LIMIT;
        while (lo < hi) {
            long mid = lo + (hi - lo) / 2;
            long count = mid / a + mid / b + mid / c - mid / ab - mid / ac - mid / bc + mid / abc;
            if (count >= n) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }
}
