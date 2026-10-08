class Solution {
public:
    bool isLongPressedName(string& name, string& typed) {
        size_t i = 0;
        for (size_t j = 0; j < typed.size(); j++) {
            char c = typed[j];
            if (i < name.size() && name[i] == c) i++;
            else if (j == 0 || typed[j - 1] != c) return false;
        }
        return i == name.size();
    }
};
