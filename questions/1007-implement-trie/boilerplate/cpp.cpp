// 1007. Implement Trie (Prefix Tree) (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Trie {
public:
    Trie() {

    }

    void insert(string word) {

    }

    bool search(string word) {
        return false;
    }

    bool startsWith(string prefix) {
        return false;
    }
};

// dojo calls this to run the tests; you don't need to change it.
class Solution {
public:
    vector<bool> runTrie(vector<string>& ops, vector<string>& words) {
        Trie trie;
        vector<bool> out;
        for (size_t i = 0; i < ops.size(); i++) {
            if (ops[i] == "insert") trie.insert(words[i]);
            else if (ops[i] == "search") out.push_back(trie.search(words[i]));
            else out.push_back(trie.startsWith(words[i]));
        }
        return out;
    }
};
