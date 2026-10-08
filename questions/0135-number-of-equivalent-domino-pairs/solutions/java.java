class Solution {
    public int numEquivDominoPairs(int[][] dominoes) {
        int[] seen = new int[100];
        int pairs = 0;
        for (int[] d : dominoes) {
            int key = 10 * Math.min(d[0], d[1]) + Math.max(d[0], d[1]);
            pairs += seen[key];
            seen[key]++;
        }
        return pairs;
    }
}
