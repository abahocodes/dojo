class Solution {
    public int findMaxConsecutiveOnes(int[] nums) {
        int best = 0, run = 0;
        for (int x : nums) {
            run = x == 1 ? run + 1 : 0;
            best = Math.max(best, run);
        }
        return best;
    }
}
