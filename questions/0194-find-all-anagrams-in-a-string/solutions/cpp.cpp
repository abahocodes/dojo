class Solution {
public:
    vector<int> findAnagrams(string& s, string& p) {
        int m = p.size(), n = s.size();
        vector<int> result;
        if (m > n) return result;
        array<int, 26> need{}, have{};
        for (char c : p) need[c - 'a']++;
        int matches = count(need.begin(), need.end(), 0);
        auto change = [&](int i, int delta) {
            if (have[i] == need[i]) matches--;
            have[i] += delta;
            if (have[i] == need[i]) matches++;
        };
        for (int j = 0; j < n; j++) {
            change(s[j] - 'a', 1);
            if (j >= m) change(s[j - m] - 'a', -1);
            if (j >= m - 1 && matches == 26) result.push_back(j - m + 1);
        }
        return result;
    }
};
