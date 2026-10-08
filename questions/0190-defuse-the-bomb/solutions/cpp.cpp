class Solution {
public:
    vector<int> decrypt(vector<int>& code, int k) {
        int n = code.size();
        vector<int> result(n, 0);
        if (k == 0) return result;
        int start = k > 0 ? 1 : n + k;
        int end = k > 0 ? k : n - 1;
        int window = 0;
        for (int j = start; j <= end; j++) window += code[j % n];
        for (int i = 0; i < n; i++) {
            result[i] = window;
            window -= code[start % n];
            start++;
            end++;
            window += code[end % n];
        }
        return result;
    }
};
