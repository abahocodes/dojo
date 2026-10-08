class Solution {
public:
    vector<int> partitionLabels(string& s) {
        unordered_map<char, int> last;
        for (int i = 0; i < (int)s.size(); i++) last[s[i]] = i;
        vector<int> sizes;
        int start = 0, end = 0;
        for (int i = 0; i < (int)s.size(); i++) {
            end = max(end, last[s[i]]);
            if (i == end) {
                sizes.push_back(i - start + 1);
                start = i + 1;
            }
        }
        return sizes;
    }
};
