class Solution {
public:
    string removeOuterParentheses(string& s) {
        string out;
        int depth = 0;
        for (char ch : s) {
            if (ch == '(') {
                if (depth > 0) out.push_back(ch);
                depth++;
            } else {
                depth--;
                if (depth > 0) out.push_back(ch);
            }
        }
        return out;
    }
};
