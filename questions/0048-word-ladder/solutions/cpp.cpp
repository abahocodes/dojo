class Solution {
public:
    int ladderLength(string& beginWord, string& endWord, vector<string>& wordList) {
        unordered_set<string> words(wordList.begin(), wordList.end());
        if (!words.count(endWord)) return 0;
        words.erase(beginWord);
        queue<pair<string, int>> q;
        q.push({beginWord, 1});
        while (!q.empty()) {
            auto [word, steps] = q.front();
            q.pop();
            for (size_t i = 0; i < word.size(); i++) {
                char original = word[i];
                for (char ch = 'a'; ch <= 'z'; ch++) {
                    word[i] = ch;
                    if (words.count(word)) {
                        if (word == endWord) return steps + 1;
                        words.erase(word);  // visited: never enqueue twice
                        q.push({word, steps + 1});
                    }
                }
                word[i] = original;
            }
        }
        return 0;
    }
};
