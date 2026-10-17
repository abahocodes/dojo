// 1008. Design Add and Search Words Data Structure (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class WordDictionary {
    public WordDictionary() {

    }

    public void addWord(String word) {

    }

    public boolean search(String word) {
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
