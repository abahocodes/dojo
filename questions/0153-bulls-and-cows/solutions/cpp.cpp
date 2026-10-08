class Solution {
public:
    string getHint(string& secret, string& guess) {
        int bulls = 0, cows = 0;
        int bal[10] = {0};
        for (size_t i = 0; i < secret.size(); i++) {
            int a = secret[i] - '0';
            int b = guess[i] - '0';
            if (a == b) { bulls++; continue; }
            if (bal[a] < 0) cows++;
            if (bal[b] > 0) cows++;
            bal[a]++;
            bal[b]--;
        }
        return to_string(bulls) + "A" + to_string(cows) + "B";
    }
};
