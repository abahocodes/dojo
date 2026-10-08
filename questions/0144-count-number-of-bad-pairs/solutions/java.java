class Solution {
    public long countBadPairs(int[] nums) {
        Map<Integer, Integer> seen = new HashMap<>();
        long good = 0;
        for (int j = 0; j < nums.length; j++) {
            int key = nums[j] - j;
            int count = seen.getOrDefault(key, 0);
            good += count;
            seen.put(key, count + 1);
        }
        long n = nums.length;
        return n * (n - 1) / 2 - good;
    }
}
