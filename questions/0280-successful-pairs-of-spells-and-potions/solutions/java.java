class Solution {
    public int[] successfulPairs(int[] spells, int[] potions, long success) {
        int[] sorted = potions.clone();
        Arrays.sort(sorted);
        int m = sorted.length;
        int[] result = new int[spells.length];
        for (int i = 0; i < spells.length; i++) {
            long s = spells[i];
            int lo = 0, hi = m;
            while (lo < hi) {
                int mid = (lo + hi) / 2;
                // The product reaches 10^10, so compare in long.
                if (s * sorted[mid] >= success) hi = mid;
                else lo = mid + 1;
            }
            result[i] = m - lo;
        }
        return result;
    }
}
