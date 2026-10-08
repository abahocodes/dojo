class Solution {
public:
    int firstUniqChar(string& s) {
        int counts[26] = {0};
        for (char c : s) counts[c - 'a']++;
        for (int i = 0; i < (int)s.size(); i++) {
            if (counts[s[i] - 'a'] == 1) return i;
        }
        return -1;
    }
};
