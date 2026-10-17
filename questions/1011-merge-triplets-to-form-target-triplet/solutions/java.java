class Solution {
    public boolean mergeTriplets(int[][] triplets, int[] target) {
        // Merge every triplet that never exceeds the target; the rest are poison.
        int[] best = new int[3];
        for (int[] t : triplets) {
            if (t[0] <= target[0] && t[1] <= target[1] && t[2] <= target[2]) {
                for (int i = 0; i < 3; i++) best[i] = Math.max(best[i], t[i]);
            }
        }
        return Arrays.equals(best, target);
    }
}
