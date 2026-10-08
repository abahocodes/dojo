class Solution {
    public int findShortestSubArray(int[] nums) {
        Map<Integer, Integer> first = new HashMap<>();
        Map<Integer, Integer> count = new HashMap<>();
        int degree = 0, best = 0;
        for (int i = 0; i < nums.length; i++) {
            int x = nums[i];
            first.putIfAbsent(x, i);
            int c = count.merge(x, 1, Integer::sum);
            int span = i - first.get(x) + 1;
            if (c > degree || (c == degree && span < best)) {
                degree = c;
                best = span;
            }
        }
        return best;
    }
}
