class Solution {
    public int findMaxLength(int[] nums) {
        int n = nums.length;
        // balance ranges over [-n, n]; store first index at balance + n
        int[] first = new int[2 * n + 1];
        Arrays.fill(first, -2);
        first[n] = -1;
        int balance = 0, best = 0;
        for (int i = 0; i < n; i++) {
            balance += nums[i] == 1 ? 1 : -1;
            int slot = balance + n;
            if (first[slot] != -2) best = Math.max(best, i - first[slot]);
            else first[slot] = i;
        }
        return best;
    }
}
