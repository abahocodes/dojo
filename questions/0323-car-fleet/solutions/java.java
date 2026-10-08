class Solution {
    public int carFleet(int target, int[] position, int[] speed) {
        int n = position.length;
        long[] cars = new long[n];
        for (int i = 0; i < n; i++) {
            // position in the high bits, index in the low bits: sorting orders by position
            cars[i] = ((long) position[i] << 20) | i;
        }
        Arrays.sort(cars);
        int fleets = 0;
        long leadDist = 0, leadSpeed = 1;
        for (int k = n - 1; k >= 0; k--) {
            int i = (int) (cars[k] & ((1 << 20) - 1));
            long dist = target - position[i];
            if (dist * leadSpeed > leadDist * speed[i]) {
                fleets++;
                leadDist = dist;
                leadSpeed = speed[i];
            }
        }
        return fleets;
    }
}
