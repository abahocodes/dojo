class Solution {
    struct TrieNode {
        int children[26]; // index into the trie vector, or -1
        bool isRoot = false;
        TrieNode() {
            for (int& child : children) child = -1;
        }
    };

public:
    string replaceWords(vector<string>& roots, string& sentence) {
        vector<TrieNode> trie(1);
        for (const string& root : roots) {
            int node = 0;
            for (char ch : root) {
                int k = ch - 'a';
                if (trie[node].children[k] < 0) {
                    trie[node].children[k] = trie.size();
                    trie.emplace_back();
                }
                node = trie[node].children[k];
            }
            trie[node].isRoot = true;
        }

        string out;
        size_t start = 0;
        while (start <= sentence.size()) {
            size_t stop = sentence.find(' ', start);
            if (stop == string::npos) stop = sentence.size();
            // replace the word sentence[start, stop) by its shortest root
            size_t take = stop - start;
            int node = 0;
            for (size_t i = start; i < stop; i++) {
                node = trie[node].children[sentence[i] - 'a'];
                if (node < 0) break;
                if (trie[node].isRoot) {
                    take = i - start + 1;
                    break;
                }
            }
            if (!out.empty()) out += ' ';
            out.append(sentence, start, take);
            start = stop + 1;
        }
        return out;
    }
};
