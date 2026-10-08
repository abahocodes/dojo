class Solution {
    public int smallestDistancePair(int[] nums, int k) {
        int[] sorted = nums.clone();
        Arrays.sort(sorted);
        int lo = 0, hi = sorted[sorted.length - 1] - sorted[0];
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (pairsWithin(sorted, mid) >= k) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

    private long pairsWithin(int[] sorted, int limit) {
        long count = 0;
        int left = 0;
        for (int right = 0; right < sorted.length; right++) {
            while (sorted[right] - sorted[left] > limit) left++;
            count += right - left;
        }
        return count;
    }
}
