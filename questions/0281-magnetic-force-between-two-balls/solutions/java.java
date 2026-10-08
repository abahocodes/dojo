class Solution {
    public int maxMinDistance(int[] position, int m) {
        int[] pos = position.clone();
        Arrays.sort(pos);
        int lo = 1, hi = (pos[pos.length - 1] - pos[0]) / (m - 1);
        while (lo < hi) {
            int mid = lo + (hi - lo + 1) / 2;
            if (fits(pos, m, mid)) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }

    // Greedily drop a ball in the leftmost basket at least `gap` past the last one.
    private boolean fits(int[] pos, int m, int gap) {
        int placed = 1, last = pos[0];
        for (int i = 1; i < pos.length; i++) {
            if (pos[i] - last >= gap) {
                placed++;
                last = pos[i];
                if (placed == m) return true;
            }
        }
        return false;
    }
}
