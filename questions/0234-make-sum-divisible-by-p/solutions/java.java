class Solution {
    public int minSubarrayRemove(int[] nums, int p) {
        long need = 0;
        for (int x : nums) need = (need + x) % p;
        if (need == 0) return 0;
        Map<Long, Integer> latest = new HashMap<>();
        latest.put(0L, -1);
        long cur = 0;
        int best = nums.length;
        for (int j = 0; j < nums.length; j++) {
            cur = (cur + nums[j]) % p;
            long want = (cur - need + p) % p;
            Integer at = latest.get(want);
            if (at != null) best = Math.min(best, j - at);
            latest.put(cur, j);
        }
        return best < nums.length ? best : -1;
    }
}
