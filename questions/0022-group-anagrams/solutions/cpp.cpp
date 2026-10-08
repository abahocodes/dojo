class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& words) {
        map<array<int, 26>, int> index;
        vector<vector<string>> result;
        for (const string& w : words) {
            array<int, 26> counts{};
            for (char ch : w) counts[ch - 'a']++;
            auto it = index.find(counts);
            if (it == index.end()) {
                index.emplace(counts, (int)result.size());
                result.push_back({w});
            } else {
                result[it->second].push_back(w);
            }
        }
        return result;
    }
};
