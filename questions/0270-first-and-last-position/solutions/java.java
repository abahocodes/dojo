class Solution {
    public int[] searchRange(int[] nums, int target) {
        int first = firstAtLeast(nums, target);
        if (first == nums.length || nums[first] != target) {
            return new int[] {-1, -1};
        }
        return new int[] {first, firstAtLeast(nums, (long) target + 1) - 1};
    }

    private int firstAtLeast(int[] nums, long x) {
        int lo = 0, hi = nums.length;
        while (lo < hi) {
            int mid = (lo + hi) >>> 1;
            if (nums[mid] < x) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return lo;
    }
}
