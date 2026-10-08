class Solution {
    public int[] intersect(int[] nums1, int[] nums2) {
        int[] counts = new int[1001];
        for (int x : nums1) counts[x]++;
        int[] out = new int[Math.min(nums1.length, nums2.length)];
        int k = 0;
        for (int x : nums2) {
            if (counts[x] > 0) {
                counts[x]--;
                out[k++] = x;
            }
        }
        return Arrays.copyOf(out, k);
    }
}
