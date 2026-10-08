class Solution {
public:
    bool isIsomorphic(string& s, string& t) {
        // forward[a] / backward[b] hold the partner character + 1 (0 = unmapped).
        vector<int> forward(128, 0), backward(128, 0);
        for (size_t i = 0; i < s.size(); i++) {
            int a = (unsigned char)s[i];
            int b = (unsigned char)t[i];
            if (forward[a] == 0 && backward[b] == 0) {
                forward[a] = b + 1;
                backward[b] = a + 1;
            } else if (forward[a] != b + 1) {
                return false;
            }
        }
        return true;
    }
};
