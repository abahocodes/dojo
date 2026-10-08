class Solution {
    public int[][] insertInterval(int[][] intervals, int[] newInterval) {
        List<int[]> result = new ArrayList<>();
        int start = newInterval[0], end = newInterval[1];
        int i = 0, n = intervals.length;
        // Intervals that end strictly before the new one starts.
        while (i < n && intervals[i][1] < start) result.add(intervals[i++]);
        // Intervals that overlap or touch the new one: absorb them.
        while (i < n && intervals[i][0] <= end) {
            start = Math.min(start, intervals[i][0]);
            end = Math.max(end, intervals[i][1]);
            i++;
        }
        result.add(new int[] {start, end});
        // Intervals that start strictly after the merged one ends.
        while (i < n) result.add(intervals[i++]);
        return result.toArray(new int[0][]);
    }
}
