class Solution {
    public int[][] subsetsWithDup(int[] nums) {
        int[] sorted = nums.clone();
        Arrays.sort(sorted); // equal values become neighbours
        List<int[]> result = new ArrayList<>();
        backtrack(sorted, 0, new ArrayDeque<>(), result);
        return result.toArray(new int[0][]);
    }

    private void backtrack(int[] nums, int start, Deque<Integer> current, List<int[]> result) {
        result.add(current.stream().mapToInt(Integer::intValue).toArray());
        for (int i = start; i < nums.length; i++) {
            if (i > start && nums[i] == nums[i - 1]) continue; // already tried here
            current.addLast(nums[i]);
            backtrack(nums, i + 1, current, result);
            current.removeLast();
        }
    }
}
