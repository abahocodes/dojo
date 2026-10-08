class Solution {
    public int maximumCandies(int[] candies, long k) {
        int lo = 0, hi = 0;
        for (int c : candies) hi = Math.max(hi, c);
        while (lo < hi) {
            int mid = lo + (hi - lo + 1) / 2;
            long shares = 0;
            for (int c : candies) {
                shares += c / mid;
                if (shares >= k) break;
            }
            if (shares >= k) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }
}
