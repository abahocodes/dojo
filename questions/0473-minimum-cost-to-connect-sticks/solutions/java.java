class Solution {
    public long connectSticks(int[] sticks) {
        PriorityQueue<Long> heap = new PriorityQueue<>();
        for (int s : sticks) heap.add((long) s);
        long total = 0;
        while (heap.size() > 1) {
            long joined = heap.poll() + heap.poll();
            total += joined;
            heap.add(joined);
        }
        return total;
    }
}
