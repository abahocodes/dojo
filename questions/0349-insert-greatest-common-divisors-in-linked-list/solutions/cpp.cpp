class Solution {
public:
    ListNode* insertGcds(ListNode* head) {
        ListNode* cur = head;
        while (cur->next != nullptr) {
            ListNode* nxt = cur->next;
            cur->next = new ListNode(std::gcd(cur->val, nxt->val), nxt);
            cur = nxt;
        }
        return head;
    }
};
