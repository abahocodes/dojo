class Solution {
    public int[] kthLargestStream(int k, int[] nums, int[] adds) {
        // Min-heap holding the k largest values seen so far; its top is the answer.
        PriorityQueue<Integer> heap = new PriorityQueue<>();
        for (int v : nums) add(heap, k, v);
        int[] result = new int[adds.length];
        for (int i = 0; i < adds.length; i++) {
            add(heap, k, adds[i]);
            result[i] = heap.peek();
        }
        return result;
    }

    private void add(PriorityQueue<Integer> heap, int k, int v) {
        if (heap.size() < k) {
            heap.add(v);
        } else if (v > heap.peek()) {
            heap.poll();
            heap.add(v);
        }
    }
}
