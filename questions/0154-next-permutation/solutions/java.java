class Solution {
    public int[] nextPermutation(int[] nums) {
        int n = nums.length;
        int i = n - 2;
        while (i >= 0 && nums[i] >= nums[i + 1]) i--;
        if (i >= 0) {
            int j = n - 1;
            while (nums[j] <= nums[i]) j--;
            int tmp = nums[i];
            nums[i] = nums[j];
            nums[j] = tmp;
        }
        for (int lo = i + 1, hi = n - 1; lo < hi; lo++, hi--) {
            int tmp = nums[lo];
            nums[lo] = nums[hi];
            nums[hi] = tmp;
        }
        return nums;
    }
}
