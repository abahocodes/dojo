class Solution {
public:
    int countBinarySubstrings(string& s) {
        int total = 0, prevRun = 0, curRun = 1;
        for (size_t i = 1; i < s.size(); i++) {
            if (s[i] == s[i - 1]) curRun++;
            else {
                total += min(prevRun, curRun);
                prevRun = curRun;
                curRun = 1;
            }
        }
        return total + min(prevRun, curRun);
    }
};
