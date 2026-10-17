class Solution {
public:
    vector<string> letterCombinations(string& digits) {
        vector<string> result;
        if (digits.empty()) return result;
        string current;
        backtrack(digits, 0, current, result);
        return result;
    }

private:
    const vector<string> keys = {"", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"};

    void backtrack(const string& digits, int i, string& current, vector<string>& result) {
        if (i == (int)digits.size()) {
            result.push_back(current);
            return;
        }
        for (char letter : keys[digits[i] - '0']) {
            current.push_back(letter);
            backtrack(digits, i + 1, current, result);
            current.pop_back();
        }
    }
};
