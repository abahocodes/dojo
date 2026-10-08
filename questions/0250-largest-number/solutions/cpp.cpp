class Solution {
public:
    string largestNumber(vector<int>& nums) {
        vector<string> parts;
        for (int x : nums) parts.push_back(to_string(x));
        sort(parts.begin(), parts.end(), [](const string& a, const string& b) {
            return a + b > b + a;
        });
        if (parts[0] == "0") return "0";
        string out;
        for (const string& p : parts) out += p;
        return out;
    }
};
