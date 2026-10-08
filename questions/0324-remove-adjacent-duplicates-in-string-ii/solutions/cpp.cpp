class Solution {
public:
    string removeKDuplicates(string& s, int k) {
        vector<pair<char, int>> stack;
        for (char ch : s) {
            if (!stack.empty() && stack.back().first == ch) {
                if (++stack.back().second == k) {
                    stack.pop_back();
                }
            } else {
                stack.push_back({ch, 1});
            }
        }
        string result;
        for (auto& [ch, count] : stack) {
            result.append(count, ch);
        }
        return result;
    }
};
