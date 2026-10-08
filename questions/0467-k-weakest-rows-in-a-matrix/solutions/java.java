class Solution {
    public int[] kWeakestRows(int[][] mat, int k) {
        int rows = mat.length;
        int[] counts = new int[rows];
        Integer[] order = new Integer[rows];
        for (int i = 0; i < rows; i++) {
            counts[i] = soldiers(mat[i]);
            order[i] = i;
        }
        Arrays.sort(order, (a, b) -> counts[a] != counts[b] ? Integer.compare(counts[a], counts[b]) : Integer.compare(a, b));
        int[] result = new int[k];
        for (int i = 0; i < k; i++) result[i] = order[i];
        return result;
    }

    // Rows are 1s then 0s: binary search for the first 0.
    private int soldiers(int[] row) {
        int lo = 0, hi = row.length;
        while (lo < hi) {
            int mid = (lo + hi) >>> 1;
            if (row[mid] == 1) lo = mid + 1;
            else hi = mid;
        }
        return lo;
    }
}
