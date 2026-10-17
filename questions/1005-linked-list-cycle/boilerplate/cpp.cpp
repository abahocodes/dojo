// 1005. Linked List Cycle (easy)
// /show to reread the problem · /test runs visible tests · /submit when done

/**
 * Provided by dojo:
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

class Solution {
public:
    bool hasCycle(ListNode* head) {
        return false;
    }

    // dojo calls this to run the tests; you don't need to change it.
    bool runHasCycle(vector<int>& values, int pos) {
        vector<ListNode*> nodes;
        for (int v : values) nodes.push_back(new ListNode(v));
        for (size_t i = 0; i + 1 < nodes.size(); i++) nodes[i]->next = nodes[i + 1];
        if (pos >= 0) nodes.back()->next = nodes[pos];
        bool result = hasCycle(nodes.empty() ? nullptr : nodes[0]);
        for (ListNode* node : nodes) delete node;
        return result;
    }
};
