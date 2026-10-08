class Solution {
public:
    string countOfAtoms(string& formula) {
        size_t n = formula.size(), i = 0;
        auto readNumber = [&]() -> long long {
            size_t start = i;
            while (i < n && isdigit((unsigned char)formula[i])) i++;
            return i > start ? stoll(formula.substr(start, i - start)) : 1LL;
        };
        vector<map<string, long long>> stk(1);  // one map per open group
        while (i < n) {
            char ch = formula[i];
            if (ch == '(') {
                stk.emplace_back();
                i++;
            } else if (ch == ')') {
                i++;
                long long mult = readNumber();
                map<string, long long> group = move(stk.back());
                stk.pop_back();
                for (auto& [name, cnt] : group) stk.back()[name] += cnt * mult;
            } else {
                size_t start = i++;
                while (i < n && islower((unsigned char)formula[i])) i++;
                string name = formula.substr(start, i - start);
                stk.back()[name] += readNumber();
            }
        }
        string out;
        for (auto& [name, cnt] : stk[0]) {  // std::map iterates in sorted order
            out += name;
            if (cnt > 1) out += to_string(cnt);
        }
        return out;
    }
};
