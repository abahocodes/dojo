class Solution {
public:
    bool wordPattern(string& pattern, string& s) {
        vector<string> words;
        istringstream in(s);
        string w;
        while (in >> w) words.push_back(w);
        if (words.size() != pattern.size()) return false;
        unordered_map<char, string> letterToWord;
        unordered_map<string, char> wordToLetter;
        for (size_t i = 0; i < words.size(); i++) {
            char c = pattern[i];
            auto it = letterToWord.find(c);
            if (it == letterToWord.end()) {
                if (wordToLetter.count(words[i])) return false;
                letterToWord[c] = words[i];
                wordToLetter[words[i]] = c;
            } else if (it->second != words[i]) {
                return false;
            }
        }
        return true;
    }
};
