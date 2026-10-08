class Solution {
    public int minMovesComplementary(int[] nums, int limit) {
        int n = nums.length;
        int[] delta = new int[2 * limit + 2];
        for (int i = 0; i < n / 2; i++) {
            int a = nums[i], b = nums[n - 1 - i];
            int lo = Math.min(a, b), hi = Math.max(a, b);
            delta[2] += 2;
            delta[lo + 1] -= 1;
            delta[hi + limit + 1] += 1;
            delta[a + b] -= 1;
            delta[a + b + 1] += 1;
        }
        int best = n, moves = 0;
        for (int t = 2; t <= 2 * limit; t++) {
            moves += delta[t];
            best = Math.min(best, moves);
        }
        return best;
    }
}
