class Solution {
public:
    int nthMagicalNumber(int n, int a, int b) {
        const long long MOD = 1000000007LL;
        long long lcm = (long long)a / std::gcd(a, b) * b;
        long long lo = min(a, b), hi = (long long)n * min(a, b);
        while (lo < hi) {
            long long mid = lo + (hi - lo) / 2;
            if (mid / a + mid / b - mid / lcm >= n) hi = mid;
            else lo = mid + 1;
        }
        return (int)(lo % MOD);
    }
};
