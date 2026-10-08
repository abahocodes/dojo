class Solution {
    public int maximumRemovals(String s, String p, int[] removable) {
        int[] removedAt = new int[s.length()];
        Arrays.fill(removedAt, removable.length);
        for (int step = 0; step < removable.length; step++) removedAt[removable[step]] = step;
        int lo = 0, hi = removable.length;
        while (lo < hi) {
            int mid = lo + (hi - lo + 1) / 2;
            int j = 0;
            for (int i = 0; i < s.length() && j < p.length(); i++) {
                if (removedAt[i] >= mid && s.charAt(i) == p.charAt(j)) j++;
            }
            if (j == p.length()) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }
}
