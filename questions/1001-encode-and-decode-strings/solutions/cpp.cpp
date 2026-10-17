class Solution {
public:
    string encode(vector<string>& strs) {
        // Each string becomes "<length>#<string>"; the length says where it ends.
        string out;
        for (const string& s : strs) {
            out += to_string(s.size());
            out += '#';
            out += s;
        }
        return out;
    }

    vector<string> decode(string s) {
        vector<string> result;
        size_t i = 0;
        while (i < s.size()) {
            size_t j = s.find('#', i);
            size_t length = stoul(s.substr(i, j - i));
            result.push_back(s.substr(j + 1, length));
            i = j + 1 + length;
        }
        return result;
    }

    // dojo calls this to run the tests; you don't need to change it.
    vector<string> encodeAndDecode(vector<string>& strs) {
        vector<string> copy = strs;
        string encoded = encode(copy);
        vector<string> other(strs.rbegin(), strs.rend());
        other.push_back("dojo");
        encode(other);
        return decode(encoded);
    }
};
