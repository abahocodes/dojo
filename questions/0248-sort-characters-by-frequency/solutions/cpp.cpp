class Solution {
public:
    string frequencySort(string& s) {
        vector<int> counts(128, 0);
        for (unsigned char c : s) counts[c]++;
        vector<int> chars;
        for (int c = 0; c < 128; c++) if (counts[c] > 0) chars.push_back(c);
        sort(chars.begin(), chars.end(), [&](int a, int b) {
            return counts[a] != counts[b] ? counts[a] > counts[b] : a < b;
        });
        string out;
        out.reserve(s.size());
        for (int c : chars) out.append(counts[c], (char)c);
        return out;
    }
};
