class Solution {
public:
    int numRescueBoats(vector<int>& people, int limit) {
        vector<int> p(people);
        sort(p.begin(), p.end());
        int i = 0, j = (int)p.size() - 1, boats = 0;
        while (i <= j) {
            if (p[i] + p[j] <= limit) i++;
            j--;
            boats++;
        }
        return boats;
    }
};
