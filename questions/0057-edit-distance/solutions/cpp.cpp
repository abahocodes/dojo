class Solution {
public:
    int editDistance(string& word1, string& word2) {
        int m = word2.size();
        vector<int> prev(m + 1);
        for (int j = 0; j <= m; j++) prev[j] = j;
        for (int i = 1; i <= (int)word1.size(); i++) {
            char c1 = word1[i - 1];
            vector<int> curr(m + 1, 0);
            curr[0] = i;
            for (int j = 1; j <= m; j++) {
                if (c1 == word2[j - 1]) curr[j] = prev[j - 1];
                else curr[j] = 1 + min({prev[j], curr[j - 1], prev[j - 1]});
            }
            prev = move(curr);
        }
        return prev[m];
    }
};
