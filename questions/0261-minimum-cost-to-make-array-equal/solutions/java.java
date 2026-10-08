class Solution {
    public long minCost(int[] nums, int[] cost) {
        int n = nums.length;
        long[][] pairs = new long[n][2];
        long total = 0;
        for (int i = 0; i < n; i++) {
            pairs[i][0] = nums[i];
            pairs[i][1] = cost[i];
            total += cost[i];
        }
        Arrays.sort(pairs, (a, b) -> Long.compare(a[0], b[0]));
        long acc = 0;
        long target = pairs[0][0];
        for (long[] p : pairs) {
            acc += p[1];
            if (2 * acc >= total) {
                target = p[0];
                break;
            }
        }
        long answer = 0;
        for (int i = 0; i < n; i++) answer += (long) cost[i] * Math.abs(nums[i] - target);
        return answer;
    }
}
