class Solution {
    public int nthMagicalNumber(int n, int a, int b) {
        final long MOD = 1_000_000_007L;
        long lcm = (long) a / gcd(a, b) * b;
        long lo = Math.min(a, b), hi = (long) n * Math.min(a, b);
        while (lo < hi) {
            long mid = lo + (hi - lo) / 2;
            if (mid / a + mid / b - mid / lcm >= n) hi = mid;
            else lo = mid + 1;
        }
        return (int) (lo % MOD);
    }

    private int gcd(int x, int y) {
        while (y != 0) {
            int t = x % y;
            x = y;
            y = t;
        }
        return x;
    }
}
