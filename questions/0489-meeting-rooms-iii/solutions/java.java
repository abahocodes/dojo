class Solution {
    public int mostBooked(int n, int[][] meetings) {
        int[][] sorted = meetings.clone();
        Arrays.sort(sorted, (a, b) -> Integer.compare(a[0], b[0]));

        PriorityQueue<Integer> free = new PriorityQueue<>(); // free room numbers
        for (int r = 0; r < n; r++) free.offer(r);
        // Busy rooms as {end time, room}, earliest end first, then lowest room.
        // End times can pass 2^31 after repeated delays, so they are longs.
        PriorityQueue<long[]> busy = new PriorityQueue<>(
            (a, b) -> a[0] != b[0] ? Long.compare(a[0], b[0]) : Long.compare(a[1], b[1]));
        int[] count = new int[n];

        for (int[] m : sorted) {
            long start = m[0];
            long end = m[1];
            while (!busy.isEmpty() && busy.peek()[0] <= start) free.offer((int) busy.poll()[1]);
            int room;
            if (!free.isEmpty()) {
                room = free.poll();
                busy.offer(new long[] {end, room});
            } else {
                // Wait for the earliest room; it keeps the meeting's duration.
                long[] next = busy.poll();
                room = (int) next[1];
                busy.offer(new long[] {next[0] + end - start, room});
            }
            count[room]++;
        }

        int best = 0;
        for (int r = 1; r < n; r++) if (count[r] > count[best]) best = r;
        return best;
    }
}
