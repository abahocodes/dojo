class Solution {
    public boolean carPooling(int[][] trips, int capacity) {
        // change[x] = passengers boarding at km x minus passengers leaving at km x
        int[] change = new int[1001];
        for (int[] t : trips) {
            change[t[1]] += t[0];
            change[t[2]] -= t[0];
        }
        int load = 0;
        for (int delta : change) {
            load += delta;
            if (load > capacity) return false;
        }
        return true;
    }
}
