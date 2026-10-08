class Solution {
    public int findLhs(int[] nums) {
        Map<Long, Integer> count = new HashMap<>();
        for (int x : nums) count.merge((long) x, 1, Integer::sum);
        int best = 0;
        for (Map.Entry<Long, Integer> e : count.entrySet()) {
            Integer next = count.get(e.getKey() + 1);
            if (next != null) best = Math.max(best, e.getValue() + next);
        }
        return best;
    }
}
