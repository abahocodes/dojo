class Solution {
    public int minEatingSpeed(int[] piles, int h) {
        int lo = 1, hi = 0;
        for (int p : piles) hi = Math.max(hi, p);
        while (lo < hi) {
            int v = lo + (hi - lo) / 2;
            long hours = 0;
            for (int p : piles) {
                hours += (p + (long) v - 1) / v;
            }
            if (hours <= h) {
                hi = v;
            } else {
                lo = v + 1;
            }
        }
        return lo;
    }
}
