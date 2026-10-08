class Solution {
    public int leastBricks(int[][] wall) {
        Map<Integer, Integer> seams = new HashMap<>();
        int best = 0;
        for (int[] row : wall) {
            int pos = 0;
            for (int i = 0; i < row.length - 1; i++) {
                pos += row[i];
                int count = seams.merge(pos, 1, Integer::sum);
                best = Math.max(best, count);
            }
        }
        return wall.length - best;
    }
}
