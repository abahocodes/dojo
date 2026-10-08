class Solution {
    public long arrangeCoins(long n) {
        long lo = 0, hi = Math.min(n, 94906266L);
        while (lo < hi) {
            long mid = (lo + hi + 1) / 2;
            if (mid * (mid + 1) / 2 <= n) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }
}
