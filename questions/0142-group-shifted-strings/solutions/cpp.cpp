class Solution {
public:
    vector<vector<string>> groupStrings(vector<string>& strings) {
        unordered_map<string, int> index;
        vector<vector<string>> groups;
        for (const string& s : strings) {
            string key(s.size(), 'a');
            for (size_t i = 0; i < s.size(); i++) {
                key[i] = (char)('a' + (s[i] - s[0] + 26) % 26);
            }
            auto it = index.find(key);
            if (it == index.end()) {
                index[key] = (int)groups.size();
                groups.push_back({s});
            } else {
                groups[it->second].push_back(s);
            }
        }
        return groups;
    }
};
