class Solution {
    public boolean[] bookCalendar(int[][] bookings) {
        // start -> end of every accepted booking.
        TreeMap<Integer, Integer> calendar = new TreeMap<>();
        boolean[] result = new boolean[bookings.length];
        for (int k = 0; k < bookings.length; k++) {
            int start = bookings[k][0], end = bookings[k][1];
            // The latest booking that begins before `end` is the only one that can overlap.
            Map.Entry<Integer, Integer> prev = calendar.lowerEntry(end);
            if (prev != null && prev.getValue() > start) continue;
            calendar.put(start, end);
            result[k] = true;
        }
        return result;
    }
}
