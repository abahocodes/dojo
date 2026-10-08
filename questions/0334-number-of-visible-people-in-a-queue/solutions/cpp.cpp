class Solution {
public:
    vector<int> canSeePersonsCount(vector<int>& heights) {
        int n = heights.size();
        vector<int> answer(n, 0);
        vector<int> stk;  // heights, decreasing from bottom to top
        for (int i = n - 1; i >= 0; i--) {
            int seen = 0;
            while (!stk.empty() && stk.back() < heights[i]) {
                stk.pop_back();
                seen++;
            }
            if (!stk.empty()) seen++;  // the first taller person
            answer[i] = seen;
            stk.push_back(heights[i]);
        }
        return answer;
    }
};
