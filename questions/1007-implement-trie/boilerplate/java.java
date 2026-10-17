// 1007. Implement Trie (Prefix Tree) (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Trie {
    public Trie() {

    }

    public void insert(String word) {

    }

    public boolean search(String word) {
        return false;
    }

    public boolean startsWith(String prefix) {
        return false;
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
