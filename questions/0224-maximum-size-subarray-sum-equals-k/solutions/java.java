class Solution {
    public int maxSubArrayLen(int[] nums, int k) {
        Map<Long, Integer> first = new HashMap<>();
        first.put(0L, -1);
        long prefix = 0;
        int best = 0;
        for (int i = 0; i < nums.length; i++) {
            prefix += nums[i];
            Integer j = first.get(prefix - k);
            if (j != null) best = Math.max(best, i - j);
            first.putIfAbsent(prefix, i);
        }
        return best;
    }
}
