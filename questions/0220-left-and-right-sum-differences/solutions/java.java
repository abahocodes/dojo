class Solution {
    public int[] leftRightDifference(int[] nums) {
        int total = 0;
        for (int x : nums) total += x;
        int left = 0;
        int[] result = new int[nums.length];
        for (int i = 0; i < nums.length; i++) {
            int right = total - left - nums[i];
            result[i] = Math.abs(left - right);
            left += nums[i];
        }
        return result;
    }
}
