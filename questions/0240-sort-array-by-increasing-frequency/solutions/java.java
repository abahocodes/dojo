class Solution {
    public int[] frequencySortNumbers(int[] nums) {
        int[] count = new int[201];
        for (int x : nums) count[x + 100]++;
        Integer[] boxed = new Integer[nums.length];
        for (int i = 0; i < nums.length; i++) boxed[i] = nums[i];
        Arrays.sort(boxed, (a, b) -> count[a + 100] != count[b + 100]
                ? count[a + 100] - count[b + 100]
                : b - a);
        int[] result = new int[nums.length];
        for (int i = 0; i < nums.length; i++) result[i] = boxed[i];
        return result;
    }
}
