class Solution {
public:
    bool isInterleave(string& s1, string& s2, string& s3) {
        int m = s1.size(), n = s2.size();
        if (m + n != (int)s3.size()) return false;
        vector<bool> ok(n + 1, false);
        for (int i = 0; i <= m; i++) {
            for (int j = 0; j <= n; j++) {
                if (i == 0 && j == 0) {
                    ok[j] = true;
                    continue;
                }
                char c = s3[i + j - 1];
                bool fromS1 = i > 0 && ok[j] && s1[i - 1] == c;
                bool fromS2 = j > 0 && ok[j - 1] && s2[j - 1] == c;
                ok[j] = fromS1 || fromS2;
            }
        }
        return ok[n];
    }
};
