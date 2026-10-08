class Solution {
    public int[][] kSmallestPairs(int[] nums1, int[] nums2, int k) {
        // Min-heap of {sum, i, j}, ordered by sum, then i, then j. Sums need 64 bits.
        PriorityQueue<long[]> heap = new PriorityQueue<>((a, b) -> {
            if (a[0] != b[0]) return Long.compare(a[0], b[0]);
            if (a[1] != b[1]) return Long.compare(a[1], b[1]);
            return Long.compare(a[2], b[2]);
        });
        for (int i = 0; i < Math.min(k, nums1.length); i++) {
            heap.add(new long[] {(long) nums1[i] + nums2[0], i, 0});
        }
        List<int[]> result = new ArrayList<>();
        while (!heap.isEmpty() && result.size() < k) {
            long[] top = heap.poll();
            int i = (int) top[1], j = (int) top[2];
            result.add(new int[] {nums1[i], nums2[j]});
            if (j + 1 < nums2.length) {
                heap.add(new long[] {(long) nums1[i] + nums2[j + 1], i, j + 1});
            }
        }
        return result.toArray(new int[0][]);
    }
}
