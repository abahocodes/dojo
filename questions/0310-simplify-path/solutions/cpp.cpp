class Solution {
public:
    string simplifyPath(string& path) {
        vector<string> stack;
        stringstream ss(path);
        string part;
        while (getline(ss, part, '/')) {
            if (part.empty() || part == ".") continue;
            if (part == "..") {
                if (!stack.empty()) stack.pop_back();
            } else {
                stack.push_back(part);
            }
        }
        string out;
        for (const string& name : stack) out += "/" + name;
        return out.empty() ? "/" : out;
    }
};
