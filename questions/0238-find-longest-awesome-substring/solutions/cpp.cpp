class Solution {
public:
    int longestAwesome(string& s) {
        int n = s.size();
        vector<int> first(1024, n + 1);
        first[0] = 0;
        int mask = 0, best = 0;
        for (int i = 1; i <= n; i++) {
            mask ^= 1 << (s[i - 1] - '0');
            best = max(best, i - first[mask]);
            for (int d = 0; d < 10; d++) {
                best = max(best, i - first[mask ^ (1 << d)]);
            }
            if (first[mask] > i) first[mask] = i;
        }
        return best;
    }
};
