class Solution {
    public int[] moveZeroes(int[] nums) {
        int w = 0;
        for (int read = 0; read < nums.length; read++) {
            if (nums[read] != 0) {
                int tmp = nums[w];
                nums[w] = nums[read];
                nums[read] = tmp;
                w++;
            }
        }
        return nums;
    }
}
