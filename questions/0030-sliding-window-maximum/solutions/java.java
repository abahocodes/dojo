class Solution {
    public int[] maxSlidingWindow(int[] nums, int k) {
        Deque<Integer> dq = new ArrayDeque<>();
        int n = nums.length;
        int[] out = new int[Math.max(0, n - k + 1)];
        int w = 0;
        for (int i = 0; i < n; i++) {
            int x = nums[i];
            while (!dq.isEmpty() && nums[dq.peekLast()] <= x) dq.pollLast();
            dq.addLast(i);
            if (dq.peekFirst() <= i - k) dq.pollFirst();
            if (i >= k - 1) out[w++] = nums[dq.peekFirst()];
        }
        return out;
    }
}
