class Solution {
    public int numberOfNiceSubarrays(int[] nums, int k) {
        int[] seen = new int[nums.length + 1];
        seen[0] = 1;
        int odds = 0, total = 0;
        for (int x : nums) {
            odds += x & 1;
            if (odds >= k) total += seen[odds - k];
            seen[odds]++;
        }
        return total;
    }
}
