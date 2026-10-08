class Solution {
public:
    long long numDecodings(string& s) {
        long long prev = 1, curr = s[0] != '0' ? 1 : 0;
        for (size_t i = 2; i <= s.size(); i++) {
            long long next = 0;
            if (s[i - 1] != '0') next += curr;
            int pair = (s[i - 2] - '0') * 10 + (s[i - 1] - '0');
            if (pair >= 10 && pair <= 26) next += prev;
            prev = curr;
            curr = next;
        }
        return curr;
    }
};
