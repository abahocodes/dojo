class Solution {
    public int minKBitFlips(int[] nums, int k) {
        int n = nums.length;
        int[] ends = new int[n + 1];
        int active = 0, flips = 0;
        for (int i = 0; i < n; i++) {
            active ^= ends[i];
            if ((nums[i] ^ active) == 0) {
                if (i + k > n) return -1;
                flips++;
                active ^= 1;
                ends[i + k] ^= 1;
            }
        }
        return flips;
    }
}
