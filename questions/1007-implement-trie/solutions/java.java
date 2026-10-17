class Trie {
    private static class Node {
        Node[] next = new Node[26];
        boolean isEnd;
    }

    private final Node root = new Node();

    public Trie() {}

    public void insert(String word) {
        Node node = root;
        for (int i = 0; i < word.length(); i++) {
            int c = word.charAt(i) - 'a';
            if (node.next[c] == null) node.next[c] = new Node();
            node = node.next[c];
        }
        node.isEnd = true;
    }

    // The node spelled by s, or null if the path leaves the trie.
    private Node walk(String s) {
        Node node = root;
        for (int i = 0; i < s.length() && node != null; i++) {
            node = node.next[s.charAt(i) - 'a'];
        }
        return node;
    }

    public boolean search(String word) {
        Node node = walk(word);
        return node != null && node.isEnd;
    }

    public boolean startsWith(String prefix) {
        return walk(prefix) != null;
    }
}

// dojo calls this to run the tests; you don't need to change it.
class Solution {
    public boolean[] runTrie(String[] ops, String[] words) {
        Trie trie = new Trie();
        List<Boolean> out = new ArrayList<>();
        for (int i = 0; i < ops.length; i++) {
            if (ops[i].equals("insert")) trie.insert(words[i]);
            else if (ops[i].equals("search")) out.add(trie.search(words[i]));
            else out.add(trie.startsWith(words[i]));
        }
        boolean[] result = new boolean[out.size()];
        for (int i = 0; i < result.length; i++) result[i] = out.get(i);
        return result;
    }
}
