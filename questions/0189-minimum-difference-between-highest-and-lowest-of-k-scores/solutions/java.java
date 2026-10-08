class Solution {
    public int minimumDifference(int[] nums, int k) {
        int[] s = nums.clone();
        Arrays.sort(s);
        int best = Integer.MAX_VALUE;
        for (int i = 0; i + k - 1 < s.length; i++) {
            best = Math.min(best, s[i + k - 1] - s[i]);
        }
        return best;
    }
}
