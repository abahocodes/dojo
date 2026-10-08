class Solution {
public:
    vector<vector<string>> suggestedProducts(vector<string>& products, string& searchWord) {
        vector<string> ordered = products;
        sort(ordered.begin(), ordered.end());
        vector<vector<string>> result;
        auto start = ordered.begin();
        string prefix;
        for (char ch : searchWord) {
            prefix.push_back(ch);
            // First word >= prefix; longer prefixes never move it back.
            start = lower_bound(start, ordered.end(), prefix);
            vector<string> suggestions;
            for (auto it = start; it != ordered.end() && it - start < 3; ++it) {
                if (it->compare(0, prefix.size(), prefix) != 0) break;
                suggestions.push_back(*it);
            }
            result.push_back(move(suggestions));
        }
        return result;
    }
};
