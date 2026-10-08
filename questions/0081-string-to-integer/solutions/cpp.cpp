class Solution {
public:
    int stringToInteger(string& s) {
        int n = s.size();
        int i = 0;
        while (i < n && s[i] == ' ') i++;
        int sign = 1;
        if (i < n && (s[i] == '+' || s[i] == '-')) {
            if (s[i] == '-') sign = -1;
            i++;
        }
        long long value = 0;
        while (i < n && s[i] >= '0' && s[i] <= '9') {
            value = value * 10 + (s[i] - '0');
            if (value > INT_MAX) return sign == 1 ? INT_MAX : INT_MIN; // stop early: the result is clamped anyway
            i++;
        }
        return (int)(sign * value);
    }
};
