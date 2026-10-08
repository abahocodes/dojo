class Solution {
    public boolean isPerfectSquare(long num) {
        long lo = 1, hi = Math.min(num, 1L << 26);
        while (lo <= hi) {
            long mid = (lo + hi) / 2;
            long square = mid * mid;
            if (square == num) return true;
            if (square < num) lo = mid + 1;
            else hi = mid - 1;
        }
        return false;
    }
}
