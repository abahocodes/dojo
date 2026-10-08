class Solution {
    public int minimizedMaximum(int n, int[] quantities) {
        int lo = 1, hi = 1;
        for (int q : quantities) hi = Math.max(hi, q);
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (storesNeeded(quantities, mid) <= n) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

    // With cap = 1 the total reaches m * 10^5, beyond int range, so use long.
    private long storesNeeded(int[] quantities, int cap) {
        long s = 0;
        for (int q : quantities) s += (q + cap - 1) / cap;
        return s;
    }
}
