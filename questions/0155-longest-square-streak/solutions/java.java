class Solution {
    public int longestSquareStreak(int[] nums) {
        int max = 0;
        for (int x : nums) max = Math.max(max, x);
        boolean[] present = new boolean[max + 1];
        for (int x : nums) present[x] = true;
        int best = -1;
        for (int x = 2; x <= max; x++) {
            if (!present[x]) continue;
            int length = 1;
            long cur = x;
            while (cur * cur <= max && present[(int) (cur * cur)]) {
                cur *= cur;
                length++;
            }
            if (length >= 2 && length > best) best = length;
        }
        return best;
    }
}
