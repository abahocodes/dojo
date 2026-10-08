class Solution {
    public int countCombinations(int[] coins, int amount) {
        // Intermediate counts can outgrow 64 bits, but sums wrap modulo 2^64,
        // so the final count (which fits in an int) still comes out exact.
        long[] ways = new long[amount + 1];
        ways[0] = 1;
        for (int c : coins) {
            for (int x = c; x <= amount; x++) {
                ways[x] += ways[x - c];
            }
        }
        return (int) ways[amount];
    }
}
