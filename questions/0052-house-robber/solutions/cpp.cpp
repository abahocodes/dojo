class Solution {
public:
    int rob(vector<int>& nums) {
        int prev = 0, curr = 0;
        for (int x : nums) {
            int next = max(curr, prev + x);
            prev = curr;
            curr = next;
        }
        return curr;
    }
};
