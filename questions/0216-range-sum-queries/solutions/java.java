class Solution {
    public long[] rangeSums(int[] nums, int[][] queries) {
        long[] prefix = new long[nums.length + 1];
        for (int i = 0; i < nums.length; i++) prefix[i + 1] = prefix[i] + nums[i];
        long[] out = new long[queries.length];
        for (int i = 0; i < queries.length; i++) {
            out[i] = prefix[queries[i][1] + 1] - prefix[queries[i][0]];
        }
        return out;
    }
}
