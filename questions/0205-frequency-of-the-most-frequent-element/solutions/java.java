class Solution {
    public int maxFrequency(int[] nums, int k) {
        int[] a = nums.clone();
        Arrays.sort(a);
        int left = 0;
        long window = 0;
        int best = 0;
        for (int right = 0; right < a.length; right++) {
            window += a[right];
            while ((long) a[right] * (right - left + 1) - window > k) window -= a[left++];
            best = Math.max(best, right - left + 1);
        }
        return best;
    }
}
