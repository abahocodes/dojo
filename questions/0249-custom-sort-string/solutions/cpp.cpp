class Solution {
public:
    string customSortString(string& order, string& s) {
        vector<int> counts(26, 0);
        for (char c : s) counts[c - 'a']++;
        vector<bool> ranked(26, false);
        string out;
        out.reserve(s.size());
        for (char c : order) {
            ranked[c - 'a'] = true;
            out.append(counts[c - 'a'], c);
        }
        for (char c : s) {
            if (!ranked[c - 'a']) out.push_back(c);
        }
        return out;
    }
};
