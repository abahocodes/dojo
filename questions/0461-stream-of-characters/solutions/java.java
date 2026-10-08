class Solution {
    private static class Node {
        Node[] next = new Node[26];
        boolean isEnd;
    }

    public boolean[] streamChecker(String[] words, String stream) {
        // Trie of reversed words.
        Node root = new Node();
        int longest = 0;
        for (String word : words) {
            longest = Math.max(longest, word.length());
            Node node = root;
            for (int i = word.length() - 1; i >= 0; i--) {
                int c = word.charAt(i) - 'a';
                if (node.next[c] == null) node.next[c] = new Node();
                node = node.next[c];
            }
            node.isEnd = true;
        }

        boolean[] result = new boolean[stream.length()];
        for (int i = 0; i < stream.length(); i++) {
            Node node = root;
            for (int j = i; j >= 0 && j > i - longest; j--) {
                node = node.next[stream.charAt(j) - 'a'];
                if (node == null) break;
                if (node.isEnd) {
                    result[i] = true;
                    break;
                }
            }
        }
        return result;
    }
}
