class Solution {
public:
    int longestValidParentheses(string& s) {
        vector<int> stk = {-1};  // bottom entry is the last unmatched position
        int best = 0;
        for (int i = 0; i < (int)s.size(); i++) {
            if (s[i] == '(') {
                stk.push_back(i);
            } else {
                stk.pop_back();
                if (stk.empty()) {
                    stk.push_back(i);
                } else {
                    best = max(best, i - stk.back());
                }
            }
        }
        return best;
    }
};
