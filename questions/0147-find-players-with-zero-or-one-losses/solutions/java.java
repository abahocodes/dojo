class Solution {
    public int[][] findWinners(int[][] matches) {
        int maxId = 0;
        for (int[] m : matches) maxId = Math.max(maxId, Math.max(m[0], m[1]));
        int[] losses = new int[maxId + 1];
        Arrays.fill(losses, -1); // -1: never played
        for (int[] m : matches) {
            if (losses[m[0]] < 0) losses[m[0]] = 0;
            losses[m[1]] = Math.max(losses[m[1]], 0) + 1;
        }
        List<Integer> never = new ArrayList<>();
        List<Integer> once = new ArrayList<>();
        for (int p = 1; p <= maxId; p++) {
            if (losses[p] == 0) never.add(p);
            else if (losses[p] == 1) once.add(p);
        }
        return new int[][] {
            never.stream().mapToInt(Integer::intValue).toArray(),
            once.stream().mapToInt(Integer::intValue).toArray()
        };
    }
}
