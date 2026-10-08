class Solution {
public:
    ListNode* doubleIt(ListNode* head) {
        if (head->val >= 5) head = new ListNode(0, head);
        for (ListNode* node = head; node != nullptr; node = node->next) {
            node->val = (node->val * 2) % 10;
            if (node->next != nullptr && node->next->val >= 5) node->val += 1;
        }
        return head;
    }
};
