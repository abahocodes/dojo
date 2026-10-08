class Solution {
public:
    vector<int> findSubstring(string& s, vector<string>& words) {
        int L = words[0].size();
        int m = words.size();
        int n = s.size();
        if ((long long)m * L > n) return {};
        unordered_map<string, int> need;
        for (const string& w : words) need[w]++;
        vector<int> result;
        for (int r = 0; r < L; r++) {
            unordered_map<string, int> have;
            int left = r;
            int count = 0;
            for (int right = r; right + L <= n; right += L) {
                string w = s.substr(right, L);
                auto it = need.find(w);
                if (it == need.end()) {
                    have.clear();
                    count = 0;
                    left = right + L;
                    continue;
                }
                have[w]++;
                count++;
                while (have[w] > it->second) {
                    have[s.substr(left, L)]--;
                    count--;
                    left += L;
                }
                if (count == m) {
                    result.push_back(left);
                    have[s.substr(left, L)]--;
                    count--;
                    left += L;
                }
            }
        }
        sort(result.begin(), result.end());
        return result;
    }
};
