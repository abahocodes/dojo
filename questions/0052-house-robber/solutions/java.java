class Solution {
    public int rob(int[] nums) {
        int prev = 0, curr = 0;
        for (int x : nums) {
            int next = Math.max(curr, prev + x);
            prev = curr;
            curr = next;
        }
        return curr;
    }
}
