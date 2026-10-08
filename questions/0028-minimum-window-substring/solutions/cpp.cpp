class Solution {
public:
    string minWindow(string& s, string& t) {
        int need[256] = {0};
        for (unsigned char ch : t) need[ch]++;
        int missing = t.size();
        int n = s.size();
        int bestStart = 0, bestLen = n + 1;
        int left = 0;
        for (int right = 0; right < n; right++) {
            unsigned char ch = s[right];
            if (need[ch] > 0) missing--;
            need[ch]--;
            while (missing == 0) {
                if (right - left + 1 < bestLen) {
                    bestStart = left;
                    bestLen = right - left + 1;
                }
                unsigned char out = s[left];
                need[out]++;
                if (need[out] > 0) missing++;
                left++;
            }
        }
        if (bestLen > n) return "";
        return s.substr(bestStart, bestLen);
    }
};
