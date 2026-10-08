class Solution {
public:
    int calculate(string& s) {
        long long result = 0, num = 0;
        int sign = 1;
        vector<long long> stack;
        for (char ch : s) {
            if (ch >= '0' && ch <= '9') {
                num = num * 10 + (ch - '0');
            } else if (ch == '+' || ch == '-') {
                result += sign * num;
                num = 0;
                sign = ch == '+' ? 1 : -1;
            } else if (ch == '(') {
                stack.push_back(result);
                stack.push_back(sign);
                result = 0;
                sign = 1;
            } else if (ch == ')') {
                result += sign * num;
                num = 0;
                long long savedSign = stack.back();
                stack.pop_back();
                long long savedResult = stack.back();
                stack.pop_back();
                result = savedResult + savedSign * result;
            }
        }
        return (int) (result + sign * num);
    }
};
