class Solution {
    private static class TrieNode {
        TrieNode[] children = new TrieNode[26];
        boolean end;
    }

    public String replaceWords(String[] roots, String sentence) {
        TrieNode trie = new TrieNode();
        for (String root : roots) {
            TrieNode node = trie;
            for (int i = 0; i < root.length(); i++) {
                int k = root.charAt(i) - 'a';
                if (node.children[k] == null) node.children[k] = new TrieNode();
                node = node.children[k];
            }
            node.end = true;
        }

        StringBuilder out = new StringBuilder();
        for (String word : sentence.split(" ")) {
            if (out.length() > 0) out.append(' ');
            out.append(shortestRoot(trie, word));
        }
        return out.toString();
    }

    private String shortestRoot(TrieNode trie, String word) {
        TrieNode node = trie;
        for (int i = 0; i < word.length(); i++) {
            node = node.children[word.charAt(i) - 'a'];
            if (node == null) return word;
            if (node.end) return word.substring(0, i + 1);
        }
        return word;
    }
}
