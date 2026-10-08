class Solution {
    private static class Node {
        Node[] next = new Node[26];
        int count;
    }

    public int[] sumPrefixScores(String[] words) {
        // Trie where every node counts how many words pass through it.
        Node root = new Node();
        for (String word : words) {
            Node node = root;
            for (int i = 0; i < word.length(); i++) {
                int c = word.charAt(i) - 'a';
                if (node.next[c] == null) node.next[c] = new Node();
                node = node.next[c];
                node.count++;
            }
        }

        int[] result = new int[words.length];
        for (int w = 0; w < words.length; w++) {
            Node node = root;
            int total = 0;
            for (int i = 0; i < words[w].length(); i++) {
                node = node.next[words[w].charAt(i) - 'a'];
                total += node.count;
            }
            result[w] = total;
        }
        return result;
    }
}
