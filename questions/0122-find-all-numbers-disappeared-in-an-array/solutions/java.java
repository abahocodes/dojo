class Solution {
    public int[] findDisappearedNumbers(int[] nums) {
        // Mark value v as seen by making nums[v - 1] negative.
        int missing = nums.length;
        for (int x : nums) {
            int i = Math.abs(x) - 1;
            if (nums[i] > 0) {
                nums[i] = -nums[i];
                missing--;
            }
        }
        int[] out = new int[missing];
        int k = 0;
        for (int i = 0; i < nums.length; i++) {
            if (nums[i] > 0) out[k++] = i + 1;
        }
        return out;
    }
}
