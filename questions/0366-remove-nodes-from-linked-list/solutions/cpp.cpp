class Solution {
public:
    ListNode* removeNodes(ListNode* head) {
        vector<ListNode*> stack;
        for (ListNode* node = head; node != nullptr; node = node->next) {
            while (!stack.empty() && stack.back()->val < node->val) stack.pop_back();
            stack.push_back(node);
        }
        for (size_t i = 0; i + 1 < stack.size(); i++) stack[i]->next = stack[i + 1];
        stack.back()->next = nullptr;
        return stack.front();
    }
};
