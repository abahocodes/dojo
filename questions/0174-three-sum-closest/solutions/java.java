class Solution {
    public int threeSumClosest(int[] nums, int target) {
        int[] a = nums.clone();
        Arrays.sort(a);
        int n = a.length;
        int best = a[0] + a[1] + a[2];
        for (int i = 0; i < n - 2; i++) {
            int lo = i + 1, hi = n - 1;
            while (lo < hi) {
                int s = a[i] + a[lo] + a[hi];
                if (Math.abs(s - target) < Math.abs(best - target)) best = s;
                if (s < target) lo++;
                else if (s > target) hi--;
                else return s;
            }
        }
        return best;
    }
}
