class Solution {
    public int[] findErrorNums(int[] nums) {
        int n = nums.length;
        boolean[] seen = new boolean[n + 1];
        int dup = 0;
        long total = 0;
        for (int x : nums) {
            if (seen[x]) dup = x;
            seen[x] = true;
            total += x;
        }
        long expected = (long) n * (n + 1) / 2;
        int missing = (int) (expected - (total - dup));
        return new int[] {dup, missing};
    }
}
