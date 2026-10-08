class Solution {
    public int[] findDuplicates(int[] nums) {
        int[] result = new int[nums.length / 2];
        int size = 0;
        for (int i = 0; i < nums.length; i++) {
            int v = Math.abs(nums[i]);
            if (nums[v - 1] < 0) result[size++] = v;
            else nums[v - 1] = -nums[v - 1];
        }
        return Arrays.copyOf(result, size);
    }
}
