class Solution {
    private List<int[]> result;
    private int[] current;
    private boolean[] used;

    public int[][] permute(int[] nums) {
        result = new ArrayList<>();
        current = new int[nums.length];
        used = new boolean[nums.length];
        backtrack(nums, 0);
        return result.toArray(new int[0][]);
    }

    private void backtrack(int[] nums, int depth) {
        if (depth == nums.length) {
            result.add(current.clone());
            return;
        }
        for (int i = 0; i < nums.length; i++) {
            if (used[i]) continue;
            used[i] = true;
            current[depth] = nums[i];
            backtrack(nums, depth + 1);
            used[i] = false;
        }
    }
}
