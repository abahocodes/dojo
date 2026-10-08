class Solution {
public:
    bool isPerfectSquare(long long num) {
        long long lo = 1, hi = min(num, 1LL << 26);
        while (lo <= hi) {
            long long mid = (lo + hi) / 2;
            long long square = mid * mid;
            if (square == num) return true;
            if (square < num) lo = mid + 1;
            else hi = mid - 1;
        }
        return false;
    }
};
