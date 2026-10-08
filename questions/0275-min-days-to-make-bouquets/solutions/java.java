class Solution {
    public int minDays(int[] bloomDay, int m, int k) {
        if ((long) m * k > bloomDay.length) {
            return -1;
        }
        int lo = Integer.MAX_VALUE, hi = Integer.MIN_VALUE;
        for (int b : bloomDay) {
            lo = Math.min(lo, b);
            hi = Math.max(hi, b);
        }
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (bouquets(bloomDay, k, mid) >= m) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        return lo;
    }

    private int bouquets(int[] bloomDay, int k, int day) {
        int made = 0, run = 0;
        for (int b : bloomDay) {
            if (b <= day) {
                run++;
                if (run == k) {
                    made++;
                    run = 0;
                }
            } else {
                run = 0;
            }
        }
        return made;
    }
}
