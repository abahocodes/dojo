class Solution {
    public boolean[] bookCalendarDouble(int[][] bookings) {
        List<int[]> booked = new ArrayList<>();   // every accepted booking
        List<int[]> overlaps = new ArrayList<>(); // stretches already covered twice
        boolean[] result = new boolean[bookings.length];
        outer:
        for (int k = 0; k < bookings.length; k++) {
            int start = bookings[k][0], end = bookings[k][1];
            for (int[] o : overlaps) {
                if (Math.max(start, o[0]) < Math.min(end, o[1])) continue outer;
            }
            for (int[] b : booked) {
                int lo = Math.max(start, b[0]), hi = Math.min(end, b[1]);
                if (lo < hi) overlaps.add(new int[] {lo, hi});
            }
            booked.add(new int[] {start, end});
            result[k] = true;
        }
        return result;
    }
}
