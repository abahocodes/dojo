class Solution {
    public int numSubseq(int[] nums, int target) {
        final int MOD = 1_000_000_007;
        int[] a = nums.clone();
        Arrays.sort(a);
        int n = a.length;
        int[] pow2 = new int[n];
        pow2[0] = 1;
        for (int i = 1; i < n; i++) pow2[i] = (int) (pow2[i - 1] * 2L % MOD);
        long total = 0;
        int lo = 0, hi = n - 1;
        while (lo <= hi) {
            if (a[lo] + a[hi] <= target) {
                total = (total + pow2[hi - lo]) % MOD;
                lo++;
            } else {
                hi--;
            }
        }
        return (int) total;
    }
}
