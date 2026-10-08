class Solution {
public:
    int maxSatisfied(vector<int>& customers, vector<int>& grumpy, int minutes) {
        int n = customers.size();
        int base = 0, gain = 0, best = 0;
        for (int i = 0; i < n; i++) {
            if (grumpy[i] == 0) base += customers[i];
            else gain += customers[i];
            if (i >= minutes && grumpy[i - minutes] == 1) gain -= customers[i - minutes];
            best = max(best, gain);
        }
        return base + best;
    }
};
