class Solution {
public:
    int numberOfSubstrings(string& s) {
        int last[3] = {-1, -1, -1};
        int total = 0;
        for (int i = 0; i < (int)s.size(); i++) {
            last[s[i] - 'a'] = i;
            total += min(last[0], min(last[1], last[2])) + 1;
        }
        return total;
    }
};
