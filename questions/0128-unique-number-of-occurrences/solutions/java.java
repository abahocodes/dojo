class Solution {
    public boolean uniqueOccurrences(int[] arr) {
        Map<Integer, Integer> count = new HashMap<>();
        for (int x : arr) count.merge(x, 1, Integer::sum);
        return new HashSet<>(count.values()).size() == count.size();
    }
}
