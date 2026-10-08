class Solution {
    public int totalFruit(int[] fruits) {
        Map<Integer, Integer> count = new HashMap<>();
        int left = 0, best = 0;
        for (int right = 0; right < fruits.length; right++) {
            count.merge(fruits[right], 1, Integer::sum);
            while (count.size() > 2) {
                int g = fruits[left++];
                int c = count.get(g) - 1;
                if (c == 0) count.remove(g);
                else count.put(g, c);
            }
            best = Math.max(best, right - left + 1);
        }
        return best;
    }
}
