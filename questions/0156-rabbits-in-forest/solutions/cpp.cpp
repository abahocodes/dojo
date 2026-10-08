class Solution {
public:
    int numRabbits(vector<int>& answers) {
        vector<int> counts(1000, 0);
        for (int x : answers) counts[x]++;
        int total = 0;
        for (int x = 0; x < 1000; x++) {
            if (counts[x] == 0) continue;
            int size = x + 1;
            int groups = (counts[x] + size - 1) / size;
            total += groups * size;
        }
        return total;
    }
};
