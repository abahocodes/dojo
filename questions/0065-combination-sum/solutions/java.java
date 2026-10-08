class Solution {
    private int[] sorted;
    private List<int[]> result;
    private List<Integer> current;

    public int[][] combinationSum(int[] candidates, int target) {
        sorted = candidates.clone();
        Arrays.sort(sorted);
        result = new ArrayList<>();
        current = new ArrayList<>();
        backtrack(0, target);
        return result.toArray(new int[0][]);
    }

    private void backtrack(int start, int remaining) {
        if (remaining == 0) {
            result.add(current.stream().mapToInt(Integer::intValue).toArray());
            return;
        }
        for (int i = start; i < sorted.length; i++) {
            int c = sorted[i];
            if (c > remaining) break; // sorted, so every later candidate is too big as well
            current.add(c);
            backtrack(i, remaining - c); // i, not i + 1: a candidate may be reused
            current.remove(current.size() - 1);
        }
    }
}
