class Solution {
    public long pickGifts(int[] gifts, int k) {
        PriorityQueue<Integer> heap = new PriorityQueue<>(Collections.reverseOrder());
        for (int g : gifts) heap.add(g);
        for (int s = 0; s < k; s++) {
            int largest = heap.poll();
            heap.add((int) Math.sqrt(largest));
        }
        long total = 0;
        for (int g : heap) total += g;
        return total;
    }
}
