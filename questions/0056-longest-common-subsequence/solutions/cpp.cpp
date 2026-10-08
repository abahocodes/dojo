class Solution {
public:
    int longestCommonSubsequence(string& a, string& b) {
        int m = b.size();
        vector<int> prev(m + 1, 0);
        for (char ca : a) {
            vector<int> curr(m + 1, 0);
            for (int j = 1; j <= m; j++) {
                if (ca == b[j - 1]) curr[j] = prev[j - 1] + 1;
                else curr[j] = max(prev[j], curr[j - 1]);
            }
            prev = move(curr);
        }
        return prev[m];
    }
};
