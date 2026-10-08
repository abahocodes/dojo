class Solution {
public:
    int thirdMax(vector<int>& nums) {
        // LLONG_MIN marks an empty slot: no int can equal it.
        long long first = LLONG_MIN, second = LLONG_MIN, third = LLONG_MIN;
        for (int v : nums) {
            long long x = v;
            if (x == first || x == second || x == third) continue;
            if (x > first) {
                third = second; second = first; first = x;
            } else if (x > second) {
                third = second; second = x;
            } else if (x > third) {
                third = x;
            }
        }
        return (int)(third == LLONG_MIN ? first : third);
    }
};
