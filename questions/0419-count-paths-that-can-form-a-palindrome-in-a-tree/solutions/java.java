class Solution {
    public long countPalindromePaths(int[] parent, String s) {
        int n = parent.length;
        List<List<Integer>> children = new ArrayList<>();
        for (int i = 0; i < n; i++) children.add(new ArrayList<>());
        for (int v = 1; v < n; v++) children.get(parent[v]).add(v);
        int[] mask = new int[n];
        int[] order = new int[n];
        int size = 1;
        for (int i = 0; i < size; i++) {
            int v = order[i];
            for (int c : children.get(v)) {
                mask[c] = mask[v] ^ (1 << (s.charAt(c) - 'a'));
                order[size++] = c;
            }
        }
        Map<Integer, Integer> seen = new HashMap<>();
        long total = 0;
        for (int v = 0; v < n; v++) {
            int m = mask[v];
            total += seen.getOrDefault(m, 0);
            for (int b = 0; b < 26; b++) total += seen.getOrDefault(m ^ (1 << b), 0);
            seen.merge(m, 1, Integer::sum);
        }
        return total;
    }
}
