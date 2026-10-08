class Solution {
    private static class TrieNode {
        Map<Character, TrieNode> children = new HashMap<>();
        String word;
    }

    public String longestWord(String[] words) {
        TrieNode trie = new TrieNode();
        for (String word : words) {
            TrieNode node = trie;
            for (int i = 0; i < word.length(); i++) {
                node = node.children.computeIfAbsent(word.charAt(i), k -> new TrieNode());
            }
            node.word = word;
        }

        String best = "";
        Deque<TrieNode> stack = new ArrayDeque<>();
        stack.push(trie);
        while (!stack.isEmpty()) {
            TrieNode node = stack.pop();
            for (TrieNode child : node.children.values()) {
                if (child.word == null) continue; // only walk through prefixes that are words themselves
                String word = child.word;
                if (word.length() > best.length()
                        || (word.length() == best.length() && word.compareTo(best) < 0)) {
                    best = word;
                }
                stack.push(child);
            }
        }
        return best;
    }
}
