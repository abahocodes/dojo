class Solution {
public:
    int numJewelsInStones(string& jewels, string& stones) {
        bool kinds[128] = {false};
        for (char c : jewels) kinds[(unsigned char)c] = true;
        int count = 0;
        for (char c : stones) if (kinds[(unsigned char)c]) count++;
        return count;
    }
};
