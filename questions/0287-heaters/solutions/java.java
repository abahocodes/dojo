class Solution {
    public int findRadius(int[] houses, int[] heaters) {
        int[] hs = heaters.clone();
        Arrays.sort(hs);
        int best = 0;
        for (int x : houses) {
            int lo = 0, hi = hs.length;
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (hs[mid] < x) lo = mid + 1;
                else hi = mid;
            }
            int near = Integer.MAX_VALUE;
            if (lo < hs.length) near = hs[lo] - x;
            if (lo > 0) near = Math.min(near, x - hs[lo - 1]);
            best = Math.max(best, near);
        }
        return best;
    }
}
