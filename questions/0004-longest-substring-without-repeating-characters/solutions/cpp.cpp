class Solution {
public:
    int lengthOfLongestSubstring(string& s) {
        unordered_map<char, int> last;
        int left = 0, best = 0;
        for (int i = 0; i < (int)s.size(); i++) {
            auto it = last.find(s[i]);
            if (it != last.end() && it->second >= left) left = it->second + 1;
            last[s[i]] = i;
            best = max(best, i - left + 1);
        }
        return best;
    }
};
