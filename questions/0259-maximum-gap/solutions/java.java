class Solution {
    public int maximumGap(int[] nums) {
        int n = nums.length;
        if (n < 2) return 0;
        int lo = nums[0], hi = nums[0];
        for (int v : nums) {
            lo = Math.min(lo, v);
            hi = Math.max(hi, v);
        }
        if (lo == hi) return 0;
        int size = Math.max(1, (hi - lo) / (n - 1));
        int count = (hi - lo) / size + 1;
        int[] bucketMin = new int[count];
        int[] bucketMax = new int[count];
        Arrays.fill(bucketMin, Integer.MAX_VALUE);
        Arrays.fill(bucketMax, -1);
        for (int v : nums) {
            int b = (v - lo) / size;
            bucketMin[b] = Math.min(bucketMin[b], v);
            bucketMax[b] = Math.max(bucketMax[b], v);
        }
        int best = 0, prev = lo;
        for (int b = 0; b < count; b++) {
            if (bucketMax[b] < 0) continue; // empty
            best = Math.max(best, bucketMin[b] - prev);
            prev = bucketMax[b];
        }
        return best;
    }
}
