class Solution {
    public int[] removeElement(int[] nums, int val) {
        int write = 0;
        for (int x : nums) {
            if (x != val) nums[write++] = x;
        }
        return Arrays.copyOf(nums, write);
    }
}
