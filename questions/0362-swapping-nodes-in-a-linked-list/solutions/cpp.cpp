class Solution {
public:
    ListNode* swapNodes(ListNode* head, int k) {
        ListNode* first = head;
        for (int i = 1; i < k; i++) first = first->next;
        ListNode* runner = first;
        ListNode* second = head;
        while (runner->next) {
            runner = runner->next;
            second = second->next;
        }
        swap(first->val, second->val);
        return head;
    }
};
