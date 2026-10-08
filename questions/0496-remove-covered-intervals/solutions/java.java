class Solution {
    public int removeCoveredIntervals(int[][] intervals) {
        // Start ascending; for equal starts the longer interval comes first.
        int[][] ordered = intervals.clone();
        Arrays.sort(ordered, (a, b) -> a[0] != b[0] ? Integer.compare(a[0], b[0]) : Integer.compare(b[1], a[1]));
        int remaining = 0;
        int maxEnd = -1;
        for (int[] iv : ordered) {
            if (iv[1] > maxEnd) {
                remaining++;
                maxEnd = iv[1];
            }
        }
        return remaining;
    }
}
