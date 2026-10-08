class Solution {
    public int minMovesKOnes(int[] nums, int k) {
        int[] q = new int[nums.length];
        int c = 0;
        for (int i = 0; i < nums.length; i++) {
            if (nums[i] == 1) {
                q[c] = i - c;
                c++;
            }
        }
        long[] prefix = new long[c + 1];
        for (int i = 0; i < c; i++) prefix[i + 1] = prefix[i] + q[i];
        long best = Long.MAX_VALUE;
        for (int lo = 0; lo + k <= c; lo++) {
            int hi = lo + k - 1;
            int mid = lo + k / 2;
            long m = q[mid];
            long cost = m * (mid - lo) - (prefix[mid] - prefix[lo])
                    + (prefix[hi + 1] - prefix[mid + 1]) - m * (hi - mid);
            best = Math.min(best, cost);
        }
        return (int) best;
    }
}
