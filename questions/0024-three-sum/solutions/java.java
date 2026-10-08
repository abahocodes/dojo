class Solution {
    public int[][] threeSum(int[] nums) {
        int[] sorted = nums.clone();
        Arrays.sort(sorted);
        int n = sorted.length;
        List<int[]> result = new ArrayList<>();
        for (int i = 0; i < n - 2; i++) {
            int a = sorted[i];
            if (a > 0) break;
            if (i > 0 && a == sorted[i - 1]) continue;
            int lo = i + 1, hi = n - 1;
            while (lo < hi) {
                long s = (long) a + sorted[lo] + sorted[hi];
                if (s < 0) {
                    lo++;
                } else if (s > 0) {
                    hi--;
                } else {
                    result.add(new int[] {a, sorted[lo], sorted[hi]});
                    lo++;
                    hi--;
                    while (lo < hi && sorted[lo] == sorted[lo - 1]) lo++;
                }
            }
        }
        return result.toArray(new int[0][]);
    }
}
