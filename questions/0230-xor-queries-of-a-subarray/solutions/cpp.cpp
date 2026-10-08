class Solution {
public:
    vector<int> xorQueries(vector<int>& arr, vector<vector<int>>& queries) {
        vector<int> prefix(arr.size() + 1, 0);
        for (size_t i = 0; i < arr.size(); i++) prefix[i + 1] = prefix[i] ^ arr[i];
        vector<int> answers;
        answers.reserve(queries.size());
        for (const auto& q : queries) answers.push_back(prefix[q[1] + 1] ^ prefix[q[0]]);
        return answers;
    }
};
