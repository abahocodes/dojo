class Solution {
public:
    int hammingWeight(long long n) {
        int count = 0;
        while (n != 0) {
            n &= n - 1;
            count++;
        }
        return count;
    }
};
