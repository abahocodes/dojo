class Solution {
public:
    int countPrefixSuffixPairs(vector<string>& words) {
        int count = 0;
        for (size_t j = 0; j < words.size(); j++) {
            const string& w = words[j];
            for (size_t i = 0; i < j; i++) {
                const string& p = words[i];
                if (p.size() <= w.size() && w.compare(0, p.size(), p) == 0 &&
                    w.compare(w.size() - p.size(), p.size(), p) == 0) {
                    count++;
                }
            }
        }
        return count;
    }
};
