class Solution {
public:
    int pairSum(ListNode* head) {
        ListNode* slow = head;
        ListNode* fast = head;
        while (fast && fast->next) {
            slow = slow->next;
            fast = fast->next->next;
        }
        ListNode* prev = nullptr;
        while (slow) {
            ListNode* nxt = slow->next;
            slow->next = prev;
            prev = slow;
            slow = nxt;
        }
        int best = 0;
        ListNode* a = head;
        ListNode* b = prev;
        while (b) {
            best = max(best, a->val + b->val);
            a = a->next;
            b = b->next;
        }
        return best;
    }
};
