class Solution {
public:
    vector<bool> camelMatch(vector<string>& queries, string& pattern) {
        vector<bool> result;
        result.reserve(queries.size());
        for (const string& q : queries) result.push_back(matches(q, pattern));
        return result;
    }

private:
    bool matches(const string& query, const string& pattern) {
        size_t j = 0;
        for (char c : query) {
            if (j < pattern.size() && c == pattern[j]) j++;
            else if (c >= 'A' && c <= 'Z') return false;
        }
        return j == pattern.size();
    }
};
