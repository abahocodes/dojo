class Solution {
public:
    int isPrefixOfWord(string& sentence, string& searchWord) {
        istringstream in(sentence);
        string word;
        int position = 0;
        while (in >> word) {
            position++;
            if (word.compare(0, searchWord.size(), searchWord) == 0) return position;
        }
        return -1;
    }
};
