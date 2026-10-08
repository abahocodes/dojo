class Solution {
public:
    vector<int> dailyTemperatures(vector<int>& temperatures) {
        int n = temperatures.size();
        vector<int> answer(n, 0);
        vector<int> stack;
        for (int i = 0; i < n; i++) {
            while (!stack.empty() && temperatures[stack.back()] < temperatures[i]) {
                int j = stack.back();
                stack.pop_back();
                answer[j] = i - j;
            }
            stack.push_back(i);
        }
        return answer;
    }
};
