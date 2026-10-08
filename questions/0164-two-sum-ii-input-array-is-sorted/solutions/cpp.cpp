class Solution {
public:
    vector<int> twoSumSorted(vector<int>& numbers, int target) {
        int lo = 0, hi = (int)numbers.size() - 1;
        while (lo < hi) {
            int total = numbers[lo] + numbers[hi];
            if (total == target) return {lo + 1, hi + 1};
            if (total < target) lo++;
            else hi--;
        }
        return {};
    }
};
