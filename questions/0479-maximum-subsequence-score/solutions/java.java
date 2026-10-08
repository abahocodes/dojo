class Solution {
    public long maxScore(int[] nums1, int[] nums2, int k) {
        int n = nums1.length;
        Integer[] order = new Integer[n];
        for (int i = 0; i < n; i++) order[i] = i;
        Arrays.sort(order, (a, b) -> Integer.compare(nums2[b], nums2[a]));
        PriorityQueue<Integer> chosen = new PriorityQueue<>();
        long total = 0;
        long best = 0;
        for (int i : order) {
            chosen.add(nums1[i]);
            total += nums1[i];
            if (chosen.size() > k) total -= chosen.poll();
            if (chosen.size() == k) best = Math.max(best, total * nums2[i]);
        }
        return best;
    }
}
