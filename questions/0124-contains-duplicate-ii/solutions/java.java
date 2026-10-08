class Solution {
    public boolean containsNearbyDuplicate(int[] nums, int k) {
        Map<Integer, Integer> last = new HashMap<>();
        for (int i = 0; i < nums.length; i++) {
            Integer j = last.put(nums[i], i);
            if (j != null && i - j <= k) return true;
        }
        return false;
    }
}
