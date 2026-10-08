class Solution {
    public int[][] intervalIntersection(int[][] first, int[][] second) {
        List<int[]> result = new ArrayList<>();
        int i = 0, j = 0;
        while (i < first.length && j < second.length) {
            int lo = Math.max(first[i][0], second[j][0]);
            int hi = Math.min(first[i][1], second[j][1]);
            if (lo <= hi) result.add(new int[] {lo, hi});
            // The interval that ends first cannot meet anything later in the other list.
            if (first[i][1] < second[j][1]) i++;
            else j++;
        }
        return result.toArray(new int[0][]);
    }
}
