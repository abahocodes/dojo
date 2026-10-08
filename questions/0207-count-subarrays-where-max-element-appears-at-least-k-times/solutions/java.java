class Solution {
    public long countSubarraysMaxK(int[] nums, int k) {
        int m = 0;
        for (int v : nums) m = Math.max(m, v);
        int count = 0;
        int left = 0;
        long total = 0;
        for (int v : nums) {
            if (v == m) count++;
            while (count >= k) {
                if (nums[left] == m) count--;
                left++;
            }
            total += left;
        }
        return total;
    }
}
