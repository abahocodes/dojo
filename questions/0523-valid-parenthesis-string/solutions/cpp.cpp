class Solution {
public:
    bool checkValidString(string& s) {
        // lo..hi is the range of possible counts of unmatched '(' so far.
        int lo = 0, hi = 0;
        for (char ch : s) {
            if (ch == '(') {
                lo++;
                hi++;
            } else if (ch == ')') {
                lo--;
                hi--;
            } else { // '*' can be ')', nothing, or '('
                lo--;
                hi++;
            }
            if (hi < 0) return false;
            lo = max(lo, 0);
        }
        return lo == 0;
    }
};
