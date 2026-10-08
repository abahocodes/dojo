class Solution {
public:
    vector<int> nextLargerNodes(ListNode* head) {
        vector<int> vals;
        for (ListNode* node = head; node != nullptr; node = node->next) vals.push_back(node->val);
        vector<int> answer(vals.size(), 0);
        vector<int> waiting;
        for (int i = 0; i < (int) vals.size(); i++) {
            while (!waiting.empty() && vals[waiting.back()] < vals[i]) {
                answer[waiting.back()] = vals[i];
                waiting.pop_back();
            }
            waiting.push_back(i);
        }
        return answer;
    }
};
