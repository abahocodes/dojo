class Solution {
    public int findKthNumber(int m, int n, int k) {
        if (m > n) {
            int t = m;
            m = n;
            n = t;
        }
        int lo = 1, hi = m * n;
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (countAtMost(m, n, mid) >= k) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

    private long countAtMost(int m, int n, int x) {
        long total = 0;
        for (int i = 1; i <= m; i++) {
            int inRow = x / i;
            if (inRow == 0) break;
            total += Math.min(inRow, n);
        }
        return total;
    }
}
