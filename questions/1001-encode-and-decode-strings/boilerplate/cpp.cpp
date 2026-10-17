// 1001. Encode and Decode Strings (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Solution {
public:
    string encode(vector<string>& strs) {
        return "";
    }

    vector<string> decode(string s) {
        return {};
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
