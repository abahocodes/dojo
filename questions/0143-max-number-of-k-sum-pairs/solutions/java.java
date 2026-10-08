class Solution {
    public int maxOperations(int[] nums, int k) {
        Map<Integer, Integer> waiting = new HashMap<>();
        int ops = 0;
        for (int x : nums) {
            int partner = k - x;
            int count = waiting.getOrDefault(partner, 0);
            if (count > 0) {
                waiting.put(partner, count - 1);
                ops++;
            } else {
                waiting.merge(x, 1, Integer::sum);
            }
        }
        return ops;
    }
}
