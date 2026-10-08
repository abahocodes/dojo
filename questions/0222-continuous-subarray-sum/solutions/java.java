class Solution {
    public boolean checkSubarraySum(int[] nums, int k) {
        Map<Long, Integer> first = new HashMap<>();
        first.put(0L, -1);
        long rem = 0;
        for (int i = 0; i < nums.length; i++) {
            rem = (rem + nums[i]) % k;
            Integer j = first.get(rem);
            if (j != null) {
                if (i - j >= 2) return true;
            } else {
                first.put(rem, i);
            }
        }
        return false;
    }
}
