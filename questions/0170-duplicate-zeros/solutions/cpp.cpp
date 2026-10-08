class Solution {
public:
    vector<int> duplicateZeros(vector<int>& arr) {
        vector<int> out = arr;
        int n = (int)out.size();
        // shift = number of zeros strictly before index i: out[i] lands at i + shift.
        int shift = (int)count(out.begin(), out.end(), 0);
        for (int i = n - 1; i >= 0; i--) {
            if (out[i] == 0) {
                shift--;
                if (i + shift + 1 < n) out[i + shift + 1] = 0;
            }
            if (i + shift < n) out[i + shift] = out[i];
        }
        return out;
    }
};
