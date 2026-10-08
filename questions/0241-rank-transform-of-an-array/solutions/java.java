class Solution {
    public int[] arrayRankTransform(int[] arr) {
        int[] sorted = arr.clone();
        Arrays.sort(sorted);
        Map<Integer, Integer> rank = new HashMap<>();
        for (int x : sorted) {
            if (!rank.containsKey(x)) rank.put(x, rank.size() + 1);
        }
        int[] result = new int[arr.length];
        for (int i = 0; i < arr.length; i++) result[i] = rank.get(arr[i]);
        return result;
    }
}
