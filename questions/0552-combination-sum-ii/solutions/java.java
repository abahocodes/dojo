class Solution {
    public int[][] combinationSum2(int[] candidates, int target) {
        int[] sorted = candidates.clone();
        Arrays.sort(sorted); // equal values become neighbours
        List<int[]> result = new ArrayList<>();
        backtrack(sorted, 0, target, new ArrayDeque<>(), result);
        return result.toArray(new int[0][]);
    }

    private void backtrack(int[] c, int start, int remaining, Deque<Integer> current, List<int[]> result) {
        if (remaining == 0) {
            result.add(current.stream().mapToInt(Integer::intValue).toArray());
            return;
        }
        for (int i = start; i < c.length; i++) {
            if (c[i] > remaining) break; // every later candidate is too big as well
            if (i > start && c[i] == c[i - 1]) continue; // already tried here
            current.addLast(c[i]);
            backtrack(c, i + 1, remaining - c[i], current, result); // each card used once
            current.removeLast();
        }
    }
}
