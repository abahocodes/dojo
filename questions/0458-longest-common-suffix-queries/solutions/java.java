class Solution {
    public int[] stringIndices(String[] wordsContainer, String[] wordsQuery) {
        int total = 0;
        for (String w : wordsContainer) total += w.length();
        int[] children = new int[(total + 1) * 26];
        int[] best = new int[total + 1];
        int nodes = 1;
        for (int i = 0; i < wordsContainer.length; i++) {
            String w = wordsContainer[i];
            if (w.length() < wordsContainer[best[0]].length()) best[0] = i;
            int node = 0;
            for (int k = w.length() - 1; k >= 0; k--) {
                int slot = node * 26 + (w.charAt(k) - 'a');
                if (children[slot] == 0) {
                    children[slot] = nodes;
                    best[nodes] = i;
                    nodes++;
                } else if (w.length() < wordsContainer[best[children[slot]]].length()) {
                    best[children[slot]] = i;
                }
                node = children[slot];
            }
        }
        int[] result = new int[wordsQuery.length];
        for (int j = 0; j < wordsQuery.length; j++) {
            String q = wordsQuery[j];
            int node = 0;
            for (int k = q.length() - 1; k >= 0; k--) {
                int next = children[node * 26 + (q.charAt(k) - 'a')];
                if (next == 0) break;
                node = next;
            }
            result[j] = best[node];
        }
        return result;
    }
}
