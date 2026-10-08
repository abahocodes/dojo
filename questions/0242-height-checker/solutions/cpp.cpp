class Solution {
public:
    int heightChecker(vector<int>& heights) {
        int count[101] = {0};
        for (int h : heights) count[h]++;
        int mismatches = 0, expected = 1;
        for (int h : heights) {
            while (count[expected] == 0) expected++;
            if (h != expected) mismatches++;
            count[expected]--;
        }
        return mismatches;
    }
};
