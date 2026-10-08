class Solution {
public:
    int findShortestSubArray(vector<int>& nums) {
        unordered_map<int, int> first, count;
        int degree = 0, best = 0;
        for (int i = 0; i < (int)nums.size(); i++) {
            int x = nums[i];
            if (!first.count(x)) first[x] = i;
            int c = ++count[x];
            int span = i - first[x] + 1;
            if (c > degree || (c == degree && span < best)) {
                degree = c;
                best = span;
            }
        }
        return best;
    }
};
