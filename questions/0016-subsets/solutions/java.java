class Solution {
    public int[][] subsets(int[] nums) {
        List<int[]> result = new ArrayList<>();
        backtrack(nums, 0, new ArrayList<>(), result);
        return result.toArray(new int[0][]);
    }

    private void backtrack(int[] nums, int start, List<Integer> current, List<int[]> result) {
        result.add(current.stream().mapToInt(Integer::intValue).toArray());
        for (int i = start; i < nums.length; i++) {
            current.add(nums[i]);
            backtrack(nums, i + 1, current, result);
            current.remove(current.size() - 1);
        }
    }
}
