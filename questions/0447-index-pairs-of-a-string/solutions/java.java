class Solution {
    public int[][] indexPairs(String text, String[] words) {
        // Trie with 26-way child arrays; end[node] marks the end of a word.
        int total = 1;
        for (String word : words) total += word.length();
        int[][] next = new int[total][26];
        boolean[] end = new boolean[total];
        int size = 1;
        for (String word : words) {
            int node = 0;
            for (int k = 0; k < word.length(); k++) {
                int c = word.charAt(k) - 'a';
                if (next[node][c] == 0) next[node][c] = size++;
                node = next[node][c];
            }
            end[node] = true;
        }

        List<int[]> result = new ArrayList<>();
        for (int i = 0; i < text.length(); i++) {
            int node = 0;
            for (int j = i; j < text.length(); j++) {
                node = next[node][text.charAt(j) - 'a'];
                if (node == 0) break;
                if (end[node]) result.add(new int[] {i, j});
            }
        }
        return result.toArray(new int[0][]);
    }
}
