class Solution {
    public int hIndexSorted(int[] citations) {
        int n = citations.length;
        // Find the first index i where the n - i papers from i onward
        // all have at least n - i citations.
        int lo = 0, hi = n;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (citations[mid] >= n - mid) hi = mid;
            else lo = mid + 1;
        }
        return n - lo;
    }
}
