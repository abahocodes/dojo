class Solution {
public:
    int minStartValue(vector<int>& nums) {
        int total = 0;
        int low = 0;
        for (int x : nums) {
            total += x;
            low = min(low, total);
        }
        return 1 - low;
    }
};
