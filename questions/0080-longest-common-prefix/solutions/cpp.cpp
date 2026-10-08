class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        size_t len = strs[0].size(); // the prefix is strs[0][0, len)
        for (size_t k = 1; k < strs.size() && len > 0; k++) {
            const string& s = strs[k];
            size_t i = 0;
            while (i < len && i < s.size() && strs[0][i] == s[i]) i++;
            len = i;
        }
        return strs[0].substr(0, len);
    }
};
