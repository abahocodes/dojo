class Solution {
public:
    bool backspaceCompare(string& s, string& t) {
        int i = (int)s.size() - 1, j = (int)t.size() - 1;
        while (true) {
            i = prevChar(s, i);
            j = prevChar(t, j);
            if (i < 0 || j < 0) return i < 0 && j < 0;
            if (s[i] != t[j]) return false;
            i--;
            j--;
        }
    }

private:
    // Index of the next surviving character at or before i, or -1.
    int prevChar(const string& text, int i) {
        int skip = 0;
        while (i >= 0) {
            if (text[i] == '#') skip++;
            else if (skip > 0) skip--;
            else return i;
            i--;
        }
        return -1;
    }
};
