class Solution {
public:
    string mergeAlternately(string& word1, string& word2) {
        string out;
        out.reserve(word1.size() + word2.size());
        for (size_t i = 0; i < word1.size() || i < word2.size(); i++) {
            if (i < word1.size()) out += word1[i];
            if (i < word2.size()) out += word2[i];
        }
        return out;
    }
};
