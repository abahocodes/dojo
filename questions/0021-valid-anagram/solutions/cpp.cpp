class Solution {
public:
    bool validAnagram(string& s, string& t) {
        if (s.size() != t.size()) return false;
        int counts[26] = {0};
        for (size_t i = 0; i < s.size(); i++) {
            counts[s[i] - 'a']++;
            counts[t[i] - 'a']--;
        }
        for (int c : counts) {
            if (c != 0) return false;
        }
        return true;
    }
};
