class Solution {
    struct TrieNode {
        map<char, int> children;
        int word = -1; // index into words, or -1
    };

public:
    string longestWord(vector<string>& words) {
        vector<TrieNode> trie(1);
        for (int i = 0; i < (int)words.size(); i++) {
            int node = 0;
            for (char ch : words[i]) {
                auto it = trie[node].children.find(ch);
                if (it == trie[node].children.end()) {
                    int next = trie.size();
                    trie[node].children[ch] = next;
                    trie.emplace_back();
                    node = next;
                } else {
                    node = it->second;
                }
            }
            trie[node].word = i;
        }

        string best;
        vector<int> stack = {0};
        while (!stack.empty()) {
            int node = stack.back();
            stack.pop_back();
            for (const auto& [ch, child] : trie[node].children) {
                if (trie[child].word < 0) continue; // only walk through prefixes that are words themselves
                const string& word = words[trie[child].word];
                if (word.size() > best.size() || (word.size() == best.size() && word < best)) {
                    best = word;
                }
                stack.push_back(child);
            }
        }
        return best;
    }
};
