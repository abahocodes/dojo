class Solution {
public:
    vector<string> uncommonFromSentences(string& s1, string& s2) {
        vector<string> words;
        istringstream in(s1 + " " + s2);
        string w;
        while (in >> w) words.push_back(w);
        unordered_map<string, int> counts;
        for (auto& word : words) counts[word]++;
        vector<string> result;
        for (auto& word : words) {
            if (counts[word] == 1) result.push_back(word);
        }
        return result;
    }
};
