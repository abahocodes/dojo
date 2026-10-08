class Solution {
public:
    string minRemoveToMakeValid(string& s) {
        int n = s.size();
        vector<bool> removed(n, false);
        vector<int> stack;
        for (int i = 0; i < n; i++) {
            if (s[i] == '(') {
                stack.push_back(i);
            } else if (s[i] == ')') {
                if (!stack.empty()) stack.pop_back();
                else removed[i] = true;
            }
        }
        for (int i : stack) removed[i] = true;
        string out;
        out.reserve(n);
        for (int i = 0; i < n; i++) {
            if (!removed[i]) out.push_back(s[i]);
        }
        return out;
    }
};
