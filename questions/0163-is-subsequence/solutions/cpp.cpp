class Solution {
public:
    bool isSubsequence(string& s, string& t) {
        size_t i = 0;
        for (size_t j = 0; j < t.size() && i < s.size(); j++) {
            if (s[i] == t[j]) i++;
        }
        return i == s.size();
    }
};
