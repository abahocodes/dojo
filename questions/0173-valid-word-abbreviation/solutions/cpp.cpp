class Solution {
public:
    bool validWordAbbreviation(string& word, string& abbr) {
        long long n = word.size();
        size_t m = abbr.size();
        long long i = 0;
        size_t j = 0;
        while (i < n && j < m) {
            char c = abbr[j];
            if (isdigit((unsigned char)c)) {
                if (c == '0') return false;
                long long k = 0;
                while (j < m && isdigit((unsigned char)abbr[j])) {
                    k = k * 10 + (abbr[j] - '0');
                    j++;
                }
                i += k;
            } else {
                if (word[i] != c) return false;
                i++;
                j++;
            }
        }
        return i == n && j == m;
    }
};
