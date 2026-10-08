class Solution {
    public int largestAltitude(int[] gain) {
        int altitude = 0;
        int best = 0;
        for (int g : gain) {
            altitude += g;
            best = Math.max(best, altitude);
        }
        return best;
    }
}
