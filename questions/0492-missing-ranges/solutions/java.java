class Solution {
    public int[][] findMissingRanges(int[] nums, int lower, int upper) {
        List<int[]> ranges = new ArrayList<>();
        long prev = (long) lower - 1; // last value known to be present
        for (int i = 0; i <= nums.length; i++) {
            long x = i < nums.length ? nums[i] : (long) upper + 1;
            if (x - prev >= 2) ranges.add(new int[] {(int) (prev + 1), (int) (x - 1)});
            prev = x;
        }
        return ranges.toArray(new int[0][]);
    }
}
