class Solution {
public:
    vector<ListNode*> splitListToParts(ListNode* head, int k) {
        int n = 0;
        for (ListNode* p = head; p; p = p->next) n++;
        int base = n / k, extra = n % k;
        vector<ListNode*> parts(k, nullptr);
        ListNode* cur = head;
        for (int i = 0; i < k; i++) {
            int size = base + (i < extra ? 1 : 0);
            parts[i] = cur;
            for (int j = 0; j < size - 1; j++) cur = cur->next;
            if (size > 0) {
                ListNode* nxt = cur->next;
                cur->next = nullptr;
                cur = nxt;
            }
        }
        return parts;
    }
};
