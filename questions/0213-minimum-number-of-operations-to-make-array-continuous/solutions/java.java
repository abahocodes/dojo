class Solution {
    public int minOperationsContinuous(int[] nums) {
        int n = nums.length;
        int[] sorted = nums.clone();
        Arrays.sort(sorted);
        int k = 0;
        for (int i = 0; i < n; i++) {
            if (k == 0 || sorted[i] != sorted[k - 1]) sorted[k++] = sorted[i];
        }
        int best = 0;
        int j = 0;
        for (int i = 0; i < k; i++) {
            long limit = (long) sorted[i] + n - 1;
            while (j < k && sorted[j] <= limit) j++;
            best = Math.max(best, j - i);
        }
        return n - best;
    }
}
