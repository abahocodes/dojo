class Solution {
    public int fourSumCount(int[] nums1, int[] nums2, int[] nums3, int[] nums4) {
        Map<Integer, Integer> sums = new HashMap<>();
        for (int a : nums1) {
            for (int b : nums2) {
                sums.merge(a + b, 1, Integer::sum);
            }
        }
        int count = 0;
        for (int c : nums3) {
            for (int d : nums4) {
                count += sums.getOrDefault(-(c + d), 0);
            }
        }
        return count;
    }
}
