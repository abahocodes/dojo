class Solution {
public:
    string reorganizeString(string& s) {
        vector<int> counts(26, 0);
        for (char ch : s) counts[ch - 'a']++;
        int n = s.size();
        if (*max_element(counts.begin(), counts.end()) > (n + 1) / 2) return "";

        string result;
        int prev = -1;
        for (int pos = 0; pos < n; pos++) {
            int rest = n - pos - 1; // letters left after placing this one
            for (int c = 0; c < 26; c++) {
                if (counts[c] == 0 || c == prev) continue;
                counts[c]--;
                // The rest can follow c iff no letter needs more than half the
                // remaining slots, and c itself cannot take the very next slot.
                if (counts[c] <= rest / 2 &&
                    *max_element(counts.begin(), counts.end()) <= (rest + 1) / 2) {
                    result.push_back((char)('a' + c));
                    prev = c;
                    break;
                }
                counts[c]++;
            }
        }
        return result;
    }
};
