class Solution {
public:
    string decodeString(string& s) {
        vector<pair<string, int>> stack;
        string buf;
        int k = 0;
        for (char ch : s) {
            if (isdigit(static_cast<unsigned char>(ch))) {
                k = k * 10 + (ch - '0');
            } else if (ch == '[') {
                stack.push_back({std::move(buf), k});
                buf.clear();
                k = 0;
            } else if (ch == ']') {
                auto [prev, times] = std::move(stack.back());
                stack.pop_back();
                for (int t = 0; t < times; t++) prev += buf;
                buf = std::move(prev);
            } else {
                buf.push_back(ch);
            }
        }
        return buf;
    }
};
