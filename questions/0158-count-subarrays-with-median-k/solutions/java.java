class Solution {
    public int countSubarraysMedianK(int[] nums, int k) {
        int n = nums.length;
        int p = 0;
        while (nums[p] != k) p++;
        // right[b + off] counts right-side balances b.
        int off = n + 1;
        int[] right = new int[2 * n + 3];
        int bal = 0;
        right[off]++;
        for (int i = p + 1; i < n; i++) {
            bal += nums[i] > k ? 1 : -1;
            right[bal + off]++;
        }
        long total = 0;
        bal = 0;
        for (int i = p; i >= 0; i--) {
            if (i < p) bal += nums[i] > k ? 1 : -1;
            total += right[-bal + off] + right[1 - bal + off];
        }
        return (int) total;
    }
}
