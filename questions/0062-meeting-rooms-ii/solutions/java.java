class Solution {
    public int minMeetingRooms(int[][] intervals) {
        int n = intervals.length;
        int[] starts = new int[n], ends = new int[n];
        for (int i = 0; i < n; i++) {
            starts[i] = intervals[i][0];
            ends[i] = intervals[i][1];
        }
        Arrays.sort(starts);
        Arrays.sort(ends);
        int rooms = 0;
        int j = 0; // ends[j] is the earliest end time that hasn't freed a room yet
        for (int s : starts) {
            if (s >= ends[j]) j++;
            else rooms++;
        }
        return rooms;
    }
}
