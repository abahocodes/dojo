class Solution {
    public int minimumSize(int[] nums, int maxOperations) {
        int lo = 1, hi = 0;
        for (int b : nums) hi = Math.max(hi, b);
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            long ops = 0;
            for (int b : nums) {
                ops += (b - 1) / mid;
                if (ops > maxOperations) break;
            }
            if (ops <= maxOperations) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }
}
