class Solution {
public:
    ListNode* addTwoNumbersIi(ListNode* l1, ListNode* l2) {
        vector<int> a, b;
        for (ListNode* p = l1; p; p = p->next) a.push_back(p->val);
        for (ListNode* p = l2; p; p = p->next) b.push_back(p->val);
        ListNode* head = nullptr;
        int carry = 0;
        while (!a.empty() || !b.empty() || carry > 0) {
            int s = carry;
            if (!a.empty()) { s += a.back(); a.pop_back(); }
            if (!b.empty()) { s += b.back(); b.pop_back(); }
            head = new ListNode(s % 10, head);
            carry = s / 10;
        }
        return head;
    }
};
