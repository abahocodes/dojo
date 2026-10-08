class Solution {
    public int minSwapsToSort(int[] nums) {
        int n = nums.length;
        // Pack (value, index) into one long so a primitive sort orders by value.
        long[] keys = new long[n];
        for (int i = 0; i < n; i++) keys[i] = ((long) nums[i] << 17) | i;
        Arrays.sort(keys);
        int[] order = new int[n];
        for (int k = 0; k < n; k++) order[k] = (int) (keys[k] & ((1 << 17) - 1));
        boolean[] seen = new boolean[n];
        int swaps = 0;
        for (int i = 0; i < n; i++) {
            int length = 0;
            for (int j = i; !seen[j]; j = order[j]) {
                seen[j] = true;
                length++;
            }
            if (length > 0) swaps += length - 1;
        }
        return swaps;
    }
}
