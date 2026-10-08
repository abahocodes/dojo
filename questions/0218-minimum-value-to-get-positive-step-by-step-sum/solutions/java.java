class Solution {
    public int minStartValue(int[] nums) {
        int total = 0;
        int low = 0;
        for (int x : nums) {
            total += x;
            low = Math.min(low, total);
        }
        return 1 - low;
    }
}
