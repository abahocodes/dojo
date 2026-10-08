class Solution {
    public int[][] removeInterval(int[][] intervals, int[] toBeRemoved) {
        int cutLo = toBeRemoved[0], cutHi = toBeRemoved[1];
        List<int[]> result = new ArrayList<>();
        for (int[] iv : intervals) {
            int a = iv[0], b = iv[1];
            if (b <= cutLo || a >= cutHi) {
                result.add(new int[] {a, b}); // untouched
                continue;
            }
            if (a < cutLo) result.add(new int[] {a, cutLo}); // piece left of the cut
            if (b > cutHi) result.add(new int[] {cutHi, b}); // piece right of the cut
        }
        return result.toArray(new int[0][]);
    }
}
