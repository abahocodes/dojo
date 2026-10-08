class Solution {
public:
    vector<int> cycleLengthQueries(int n, vector<vector<int>>& queries) {
        vector<int> answer;
        answer.reserve(queries.size());
        for (auto& q : queries) {
            int a = q[0], b = q[1];
            int steps = 0;
            while (a != b) {
                if (a > b) a >>= 1;
                else b >>= 1;
                steps++;
            }
            answer.push_back(steps + 1);
        }
        return answer;
    }
};
