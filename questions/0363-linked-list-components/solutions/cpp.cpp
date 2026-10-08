class Solution {
public:
    int numComponents(ListNode* head, vector<int>& nums) {
        unordered_set<int> wanted(nums.begin(), nums.end());
        int count = 0;
        for (ListNode* cur = head; cur; cur = cur->next) {
            if (wanted.count(cur->val) && (!cur->next || !wanted.count(cur->next->val))) count++;
        }
        return count;
    }
};
