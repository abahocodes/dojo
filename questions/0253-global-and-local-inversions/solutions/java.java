class Solution {
    public boolean isIdealPermutation(int[] nums) {
        int best = -1; // max of nums[0..j-2]
        for (int j = 2; j < nums.length; j++) {
            best = Math.max(best, nums[j - 2]);
            if (best > nums[j]) return false;
        }
        return true;
    }
}
