class Solution {
public:
    vector<int> exclusiveTime(int n, vector<string>& logs) {
        vector<int> result(n, 0);
        vector<int> stack;
        int prev = 0;
        for (const string& entry : logs) {
            size_t first = entry.find(':');
            size_t second = entry.find(':', first + 1);
            int id = stoi(entry.substr(0, first));
            bool isStart = entry.compare(first + 1, second - first - 1, "start") == 0;
            int t = stoi(entry.substr(second + 1));
            if (isStart) {
                if (!stack.empty()) result[stack.back()] += t - prev;
                stack.push_back(id);
                prev = t;
            } else {
                result[stack.back()] += t - prev + 1;
                stack.pop_back();
                prev = t + 1;
            }
        }
        return result;
    }
};
