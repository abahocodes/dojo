class Solution {
    public int shortestSubarray(int[] nums, int k) {
        int n = nums.length;
        long[] prefix = new long[n + 1];
        for (int i = 0; i < n; i++) prefix[i + 1] = prefix[i] + nums[i];
        Deque<Integer> starts = new ArrayDeque<>();
        int best = n + 1;
        for (int j = 0; j <= n; j++) {
            while (!starts.isEmpty() && prefix[j] - prefix[starts.peekFirst()] >= k) {
                best = Math.min(best, j - starts.pollFirst());
            }
            while (!starts.isEmpty() && prefix[starts.peekLast()] >= prefix[j]) {
                starts.pollLast();
            }
            starts.addLast(j);
        }
        return best <= n ? best : -1;
    }
}
