class Solution {
    public int tupleSameProduct(int[] nums) {
        Map<Integer, Integer> seen = new HashMap<>();
        int total = 0;
        for (int i = 0; i < nums.length; i++) {
            for (int j = i + 1; j < nums.length; j++) {
                int p = nums[i] * nums[j];
                int count = seen.getOrDefault(p, 0);
                total += 8 * count;
                seen.put(p, count + 1);
            }
        }
        return total;
    }
}
