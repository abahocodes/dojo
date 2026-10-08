class Solution {
public:
    vector<string> commonChars(vector<string>& words) {
        vector<int> common(26, INT_MAX);
        for (auto& w : words) {
            vector<int> freq(26, 0);
            for (char c : w) freq[c - 'a']++;
            for (int i = 0; i < 26; i++) common[i] = min(common[i], freq[i]);
        }
        vector<string> result;
        for (int i = 0; i < 26; i++) {
            for (int j = 0; j < common[i]; j++) result.push_back(string(1, (char)('a' + i)));
        }
        return result;
    }
};
