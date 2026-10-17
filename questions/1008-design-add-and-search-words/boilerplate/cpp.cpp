// 1008. Design Add and Search Words Data Structure (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class WordDictionary {
public:
    WordDictionary() {

    }

    void addWord(string word) {

    }

    bool search(string word) {
        return false;
    }
};

// dojo calls this to run the tests; you don't need to change it.
class Solution {
public:
    vector<bool> runWordDictionary(vector<string>& ops, vector<string>& words) {
        WordDictionary wd;
        vector<bool> out;
        for (size_t i = 0; i < ops.size(); i++) {
            if (ops[i] == "addWord") wd.addWord(words[i]);
            else out.push_back(wd.search(words[i]));
        }
        return out;
    }
};
