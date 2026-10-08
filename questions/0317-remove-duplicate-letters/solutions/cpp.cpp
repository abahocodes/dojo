class Solution {
public:
    string removeDuplicateLetters(string& s) {
        int last[26] = {0};
        for (int i = 0; i < (int)s.size(); i++) last[s[i] - 'a'] = i;
        bool used[26] = {false};
        string stack;
        for (int i = 0; i < (int)s.size(); i++) {
            char c = s[i];
            if (used[c - 'a']) continue;
            while (!stack.empty() && stack.back() > c && last[stack.back() - 'a'] > i) {
                used[stack.back() - 'a'] = false;
                stack.pop_back();
            }
            stack.push_back(c);
            used[c - 'a'] = true;
        }
        return stack;
    }
};
