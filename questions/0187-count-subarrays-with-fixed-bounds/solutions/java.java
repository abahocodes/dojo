class Solution {
    public long countSubarraysFixedBounds(int[] nums, int minK, int maxK) {
        long total = 0;
        int bad = -1, lastMin = -1, lastMax = -1;
        for (int i = 0; i < nums.length; i++) {
            int v = nums[i];
            if (v < minK || v > maxK) bad = i;
            if (v == minK) lastMin = i;
            if (v == maxK) lastMax = i;
            total += Math.max(0, Math.min(lastMin, lastMax) - bad);
        }
        return total;
    }
}
