class Solution {
    public long mySqrt(long x) {
        long lo = 0, hi = Math.min(x, 1L << 26);
        while (lo < hi) {
            long mid = (lo + hi + 1) / 2;
            if (mid * mid <= x) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }
}
