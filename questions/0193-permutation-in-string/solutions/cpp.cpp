class Solution {
public:
    bool checkInclusion(string& s1, string& s2) {
        int m = s1.size(), n = s2.size();
        if (m > n) return false;
        array<int, 26> need{}, have{};
        for (char c : s1) need[c - 'a']++;
        int matches = count(need.begin(), need.end(), 0);
        auto change = [&](int i, int delta) {
            if (have[i] == need[i]) matches--;
            have[i] += delta;
            if (have[i] == need[i]) matches++;
        };
        for (int j = 0; j < n; j++) {
            change(s2[j] - 'a', 1);
            if (j >= m) change(s2[j - m] - 'a', -1);
            if (j >= m - 1 && matches == 26) return true;
        }
        return false;
    }
};
