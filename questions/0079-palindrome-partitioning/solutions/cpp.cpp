class Solution {
    vector<vector<bool>> pal;
    vector<vector<string>> result;
    vector<string> current;

    void backtrack(const string& s, int start) {
        int n = s.size();
        if (start == n) {
            result.push_back(current);
            return;
        }
        for (int end = start; end < n; end++) {
            if (pal[start][end]) {
                current.push_back(s.substr(start, end - start + 1));
                backtrack(s, end + 1);
                current.pop_back();
            }
        }
    }

public:
    vector<vector<string>> partition(string& s) {
        int n = s.size();
        // pal[i][j] is true when s[i..j] is a palindrome
        pal.assign(n, vector<bool>(n, false));
        for (int i = n - 1; i >= 0; i--) {
            for (int j = i; j < n; j++) {
                if (s[i] == s[j] && (j - i < 2 || pal[i + 1][j - 1])) pal[i][j] = true;
            }
        }
        result.clear();
        current.clear();
        backtrack(s, 0);
        return result;
    }
};
