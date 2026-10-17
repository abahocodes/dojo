class Solution {
public:
    string multiply(string num1, string num2) {
        // Schoolbook multiplication: digit i of num1 times digit j of num2 lands
        // in positions i + j and i + j + 1 of the product (counting from the left).
        if (num1 == "0" || num2 == "0") return "0";
        int m = num1.size(), n = num2.size();
        vector<int> out(m + n, 0);
        for (int i = m - 1; i >= 0; i--) {
            int a = num1[i] - '0';
            for (int j = n - 1; j >= 0; j--) {
                int total = a * (num2[j] - '0') + out[i + j + 1];
                out[i + j + 1] = total % 10;
                out[i + j] += total / 10;
            }
        }
        int start = 0;
        while (start < (int)out.size() - 1 && out[start] == 0) start++;
        string result;
        for (int i = start; i < (int)out.size(); i++) result.push_back('0' + out[i]);
        return result;
    }
};
