class Solution {
    public boolean canJump(int[] nums) {
        int furthest = 0;
        int last = nums.length - 1;
        for (int i = 0; i < nums.length; i++) {
            if (i > furthest) return false;
            furthest = Math.max(furthest, i + nums[i]);
            if (furthest >= last) return true;
        }
        return true;
    }
}
