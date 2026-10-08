class Solution {
    public int[] topKFrequent(int[] nums, int k) {
        Map<Integer, Integer> counts = new HashMap<>();
        for (int x : nums) counts.merge(x, 1, Integer::sum);

        // buckets[f] holds every value that occurs exactly f times
        List<List<Integer>> buckets = new ArrayList<>();
        for (int i = 0; i <= nums.length; i++) buckets.add(new ArrayList<>());
        for (Map.Entry<Integer, Integer> e : counts.entrySet()) {
            buckets.get(e.getValue()).add(e.getKey());
        }

        int[] result = new int[k];
        int size = 0;
        for (int freq = nums.length; freq > 0; freq--) {
            for (int value : buckets.get(freq)) {
                result[size++] = value;
                if (size == k) return result;
            }
        }
        return Arrays.copyOf(result, size);
    }
}
