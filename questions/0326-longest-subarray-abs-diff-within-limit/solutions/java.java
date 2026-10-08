class Solution {
    public int longestSubarrayLimit(int[] nums, int limit) {
        Deque<Integer> maxq = new ArrayDeque<>();
        Deque<Integer> minq = new ArrayDeque<>();
        int left = 0;
        int best = 0;
        for (int right = 0; right < nums.length; right++) {
            int x = nums[right];
            while (!maxq.isEmpty() && nums[maxq.peekLast()] < x) maxq.pollLast();
            maxq.offerLast(right);
            while (!minq.isEmpty() && nums[minq.peekLast()] > x) minq.pollLast();
            minq.offerLast(right);
            while (nums[maxq.peekFirst()] - nums[minq.peekFirst()] > limit) {
                left++;
                if (maxq.peekFirst() < left) maxq.pollFirst();
                if (minq.peekFirst() < left) minq.pollFirst();
            }
            best = Math.max(best, right - left + 1);
        }
        return best;
    }
}
