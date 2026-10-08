class Solution {
public:
    int maxCoins(vector<int>& nums) {
        vector<int> v;
        v.reserve(nums.size() + 2);
        v.push_back(1);
        v.insert(v.end(), nums.begin(), nums.end());
        v.push_back(1);
        int n = v.size();
        // best[i][j] is the most coins from bursting everything strictly between i and j
        vector<vector<int>> best(n, vector<int>(n, 0));
        for (int length = 2; length < n; length++) {
            for (int i = 0; i + length < n; i++) {
                int j = i + length;
                int edge = v[i] * v[j];
                int top = 0;
                for (int k = i + 1; k < j; k++) {
                    top = max(top, best[i][k] + edge * v[k] + best[k][j]);
                }
                best[i][j] = top;
            }
        }
        return best[0][n - 1];
    }
};
