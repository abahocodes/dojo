class Solution {
    static constexpr long long LIMIT = 2000000000LL;

    long long lcm(long long p, long long q) {
        long long r = p / gcd(p, q);
        if (r > (LIMIT + 1) / q) return LIMIT + 1;
        return min(r * q, LIMIT + 1);
    }

public:
    long long nthUglyNumber(int n, int a, int b, int c) {
        long long ab = lcm(a, b), ac = lcm(a, c), bc = lcm(b, c);
        long long abc = lcm(ab, c);
        long long lo = 1, hi = LIMIT;
        while (lo < hi) {
            long long mid = lo + (hi - lo) / 2;
            long long count = mid / a + mid / b + mid / c - mid / ab - mid / ac - mid / bc + mid / abc;
            if (count >= n) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }
};
