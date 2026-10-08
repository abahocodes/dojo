class Solution {
public:
    vector<string> shortestUniquePrefixes(vector<string>& words) {
        vector<array<int, 26>> children(1);
        children[0].fill(0);
        vector<int> count(1, 0);
        for (const string& w : words) {
            int node = 0;
            for (char ch : w) {
                int c = ch - 'a';
                if (children[node][c] == 0) {
                    children[node][c] = (int)children.size();
                    children.emplace_back();
                    children.back().fill(0);
                    count.push_back(0);
                }
                node = children[node][c];
                count[node]++;
            }
        }
        vector<string> result;
        result.reserve(words.size());
        for (const string& w : words) {
            int node = 0;
            size_t length = w.size();
            for (size_t i = 0; i < w.size(); i++) {
                node = children[node][w[i] - 'a'];
                if (count[node] == 1) {
                    length = i + 1;
                    break;
                }
            }
            result.push_back(w.substr(0, length));
        }
        return result;
    }
};
