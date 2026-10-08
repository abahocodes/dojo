class Solution {
public:
    string pushDominoes(string& dominoes) {
        int n = dominoes.size();
        string res = dominoes;
        // Virtual 'L' before the row and 'R' after it never push anything inward.
        int prevIdx = -1;
        char prev = 'L';
        for (int j = 0; j <= n; j++) {
            char cur = j == n ? 'R' : dominoes[j];
            if (cur == '.') continue;
            if (prev == cur) {
                for (int k = prevIdx + 1; k < j; k++) res[k] = cur;
            } else if (prev == 'R' && cur == 'L') {
                int lo = prevIdx + 1, hi = j - 1;
                while (lo < hi) {
                    res[lo++] = 'R';
                    res[hi--] = 'L';
                }
            }
            prevIdx = j;
            prev = cur;
        }
        return res;
    }
};
