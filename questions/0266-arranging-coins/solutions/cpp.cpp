class Solution {
public:
    long long arrangeCoins(long long n) {
        long long lo = 0, hi = min(n, 94906266LL);
        while (lo < hi) {
            long long mid = (lo + hi + 1) / 2;
            if (mid * (mid + 1) / 2 <= n) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }
};
