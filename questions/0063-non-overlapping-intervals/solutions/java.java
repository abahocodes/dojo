class Solution {
    public int eraseOverlapIntervals(int[][] intervals) {
        int[][] byEnd = intervals.clone();
        // keeping the interval that ends first leaves the most room for the rest
        Arrays.sort(byEnd, (a, b) -> Integer.compare(a[1], b[1]));
        int removed = 0;
        long lastEnd = Long.MIN_VALUE;
        for (int[] iv : byEnd) {
            if (iv[0] >= lastEnd) lastEnd = iv[1];
            else removed++;
        }
        return removed;
    }
}
