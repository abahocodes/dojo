class Solution {
public:
    string shiftingLetters(string& s, vector<vector<int>>& shifts) {
        int n = s.size();
        vector<int> diff(n + 1, 0);
        for (const auto& op : shifts) {
            int delta = op[2] == 1 ? 1 : -1;
            diff[op[0]] += delta;
            diff[op[1] + 1] -= delta;
        }
        string out(n, 'a');
        int net = 0;
        for (int i = 0; i < n; i++) {
            net += diff[i];
            int code = ((s[i] - 'a' + net) % 26 + 26) % 26;
            out[i] = (char)('a' + code);
        }
        return out;
    }
};
