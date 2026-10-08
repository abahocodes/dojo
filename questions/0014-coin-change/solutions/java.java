class Solution {
    public int coinChange(int[] coins, int amount) {
        int inf = amount + 1;
        int[] best = new int[amount + 1];
        Arrays.fill(best, inf);
        best[0] = 0;
        for (int total = 1; total <= amount; total++) {
            for (int coin : coins) {
                if (coin <= total && best[total - coin] + 1 < best[total]) {
                    best[total] = best[total - coin] + 1;
                }
            }
        }
        return best[amount] != inf ? best[amount] : -1;
    }
}
