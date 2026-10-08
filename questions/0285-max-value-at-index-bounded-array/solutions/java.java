class Solution {
    private long side(long v, long len) {
        if (len >= v - 1) return (v - 1) * v / 2 + (len - v + 1);
        return len * v - len * (len + 1) / 2;
    }

    public int maxValueAtIndex(int n, int index, int maxSum) {
        int lo = 1, hi = maxSum;
        while (lo < hi) {
            int mid = lo + (hi - lo + 1) / 2;
            long total = mid + side(mid, index) + side(mid, n - 1 - index);
            if (total <= maxSum) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }
}
