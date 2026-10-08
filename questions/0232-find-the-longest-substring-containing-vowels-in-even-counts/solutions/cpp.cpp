class Solution {
public:
    int findTheLongestSubstring(string& s) {
        vector<int> first(32, -2);
        first[0] = -1;
        int mask = 0;
        int best = 0;
        for (int i = 0; i < (int)s.size(); i++) {
            switch (s[i]) {
                case 'a': mask ^= 1; break;
                case 'e': mask ^= 2; break;
                case 'i': mask ^= 4; break;
                case 'o': mask ^= 8; break;
                case 'u': mask ^= 16; break;
                default: break;
            }
            if (first[mask] == -2) first[mask] = i;
            else best = max(best, i - first[mask]);
        }
        return best;
    }
};
