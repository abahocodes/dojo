class Solution {
public:
    bool hasGroupsSizeX(vector<int>& deck) {
        vector<int> count(10000, 0);
        for (int v : deck) count[v]++;
        int g = 0;
        for (int c : count) {
            if (c > 0) g = gcd(g, c);
        }
        return g >= 2;
    }
};
