class Solution {
public:
    string reverseParentheses(string& s) {
        int n = s.size();
        vector<int> partner(n, 0);
        vector<int> opens;
        for (int i = 0; i < n; i++) {
            if (s[i] == '(') {
                opens.push_back(i);
            } else if (s[i] == ')') {
                int j = opens.back();
                opens.pop_back();
                partner[i] = j;
                partner[j] = i;
            }
        }
        string out;
        int step = 1;
        for (int i = 0; i < n; i += step) {
            if (s[i] == '(' || s[i] == ')') {
                i = partner[i];
                step = -step;
            } else {
                out.push_back(s[i]);
            }
        }
        return out;
    }
};
