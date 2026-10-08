class Solution {
    ListNode* reverse(ListNode* node) {
        ListNode* prev = nullptr;
        while (node) {
            ListNode* nxt = node->next;
            node->next = prev;
            prev = node;
            node = nxt;
        }
        return prev;
    }

public:
    bool isPalindromeList(ListNode* head) {
        ListNode* slow = head;
        ListNode* fast = head;
        while (fast && fast->next) {
            slow = slow->next;
            fast = fast->next->next;
        }
        ListNode* tail = reverse(slow);
        bool ok = true;
        ListNode* a = head;
        ListNode* b = tail;
        while (b) {
            if (a->val != b->val) {
                ok = false;
                break;
            }
            a = a->next;
            b = b->next;
        }
        reverse(tail);
        return ok;
    }
};
