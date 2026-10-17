class WordDictionary {
    private static class Node {
        Node[] next = new Node[26];
        boolean isEnd;
    }

    private final Node root = new Node();

    public WordDictionary() {}

    public void addWord(String word) {
        Node node = root;
        for (int i = 0; i < word.length(); i++) {
            int c = word.charAt(i) - 'a';
            if (node.next[c] == null) node.next[c] = new Node();
            node = node.next[c];
        }
        node.isEnd = true;
    }

    public boolean search(String word) {
        return match(root, word, 0);
    }

    private boolean match(Node node, String word, int i) {
        if (i == word.length()) return node.isEnd;
        char ch = word.charAt(i);
        if (ch != '.') {
            Node child = node.next[ch - 'a'];
            return child != null && match(child, word, i + 1);
        }
        // A dot can be any letter: try every child.
        for (Node child : node.next) {
            if (child != null && match(child, word, i + 1)) return true;
        }
        return false;
    }
}

// dojo calls this to run the tests; you don't need to change it.
class Solution {
    public boolean[] runWordDictionary(String[] ops, String[] words) {
        WordDictionary wd = new WordDictionary();
        List<Boolean> out = new ArrayList<>();
        for (int i = 0; i < ops.length; i++) {
            if (ops[i].equals("addWord")) wd.addWord(words[i]);
            else out.add(wd.search(words[i]));
        }
        boolean[] result = new boolean[out.size()];
        for (int i = 0; i < result.length; i++) result[i] = out.get(i);
        return result;
    }
}
