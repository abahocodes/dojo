class Solution {
public:
    vector<string> removeSubfolders(vector<string>& folder) {
        vector<string> sorted = folder;
        sort(sorted.begin(), sorted.end());
        vector<string> result;
        for (const string& path : sorted) {
            if (!result.empty()) {
                const string& last = result.back();
                if (path.size() > last.size() && path.compare(0, last.size(), last) == 0 &&
                    path[last.size()] == '/') {
                    continue;
                }
            }
            result.push_back(path);
        }
        return result;
    }
};
