class Solution {
public:
    int maxChunksToSorted(vector<int>& arr) {
        vector<int> stk;  // maximum of each chunk, non-decreasing
        for (int x : arr) {
            if (stk.empty() || x >= stk.back()) {
                stk.push_back(x);
            } else {
                int biggest = stk.back();
                while (!stk.empty() && stk.back() > x) stk.pop_back();
                stk.push_back(biggest);
            }
        }
        return stk.size();
    }
};
