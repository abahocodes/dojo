class Solution {
public:
    long long mySqrt(long long x) {
        long long lo = 0, hi = min(x, 1LL << 26);
        while (lo < hi) {
            long long mid = (lo + hi + 1) / 2;
            if (mid * mid <= x) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }
};
