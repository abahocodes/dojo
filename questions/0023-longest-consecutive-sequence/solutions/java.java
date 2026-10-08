class Solution {
    public int longestConsecutiveSequence(int[] nums) {
        Set<Integer> values = new HashSet<>();
        for (int x : nums) values.add(x);
        int best = 0;
        for (int x : values) {
            if (!values.contains(x - 1)) {
                int length = 1;
                while (values.contains(x + length)) length++;
                best = Math.max(best, length);
            }
        }
        return best;
    }
}
