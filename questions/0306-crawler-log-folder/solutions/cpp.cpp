class Solution {
public:
    int minOperationsToMain(vector<string>& logs) {
        int depth = 0;
        for (const string& log : logs) {
            if (log == "../") depth = max(0, depth - 1);
            else if (log != "./") depth++;
        }
        return depth;
    }
};
