class Solution {
public:
    int maxDepthParens(string& s) {
        int depth = 0, best = 0;
        for (char ch : s) {
            if (ch == '(') best = max(best, ++depth);
            else if (ch == ')') depth--;
        }
        return best;
    }
};
