class Solution {
    public int minCostClimbingStairs(int[] cost) {
        // a, b = cheapest way to stand on step i-2, i-1 (steps 0 and 1 are free starts).
        int a = 0, b = 0;
        for (int i = 2; i <= cost.length; i++) {
            int next = Math.min(b + cost[i - 1], a + cost[i - 2]);
            a = b;
            b = next;
        }
        return b;
    }
}
