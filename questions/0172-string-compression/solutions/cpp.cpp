class Solution {
public:
    string compress(string& chars) {
        string out;
        size_t n = chars.size();
        size_t i = 0;
        while (i < n) {
            size_t j = i;
            while (j < n && chars[j] == chars[i]) j++;
            out += chars[i];
            if (j - i > 1) out += to_string(j - i);
            i = j;
        }
        return out;
    }
};
