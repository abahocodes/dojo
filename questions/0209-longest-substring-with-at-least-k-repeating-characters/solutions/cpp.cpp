class Solution {
public:
    int longestSubstringKRepeating(string& s, int k) {
        int n = s.size();
        int best = 0;
        for (int limit = 1; limit <= 26; limit++) {
            int count[26] = {0};
            int left = 0, unique = 0, atLeast = 0;
            for (int right = 0; right < n; right++) {
                int c = s[right] - 'a';
                if (count[c] == 0) unique++;
                count[c]++;
                if (count[c] == k) atLeast++;
                while (unique > limit) {
                    int d = s[left] - 'a';
                    if (count[d] == k) atLeast--;
                    count[d]--;
                    if (count[d] == 0) unique--;
                    left++;
                }
                if (unique == atLeast) best = max(best, right - left + 1);
            }
        }
        return best;
    }
};
