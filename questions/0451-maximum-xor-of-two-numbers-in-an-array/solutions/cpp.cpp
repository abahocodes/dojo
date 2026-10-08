class Solution {
public:
    int findMaximumXor(vector<int>& nums) {
        const int BITS = 31;  // every value is below 2^31
        // Binary trie in a flat array: child[2 * node + bit] is the child index, 0 = none.
        vector<int> child(2 * (nums.size() * BITS + 1), 0);
        int size = 1;
        int best = 0;
        for (int x : nums) {
            int node = 0;
            for (int b = BITS - 1; b >= 0; b--) {
                int slot = 2 * node + ((x >> b) & 1);
                if (child[slot] == 0) child[slot] = size++;
                node = child[slot];
            }
            // Walk toward the opposite bit wherever possible.
            node = 0;
            int cur = 0;
            for (int b = BITS - 1; b >= 0; b--) {
                int bit = (x >> b) & 1;
                int want = child[2 * node + (bit ^ 1)];
                if (want != 0) {
                    cur |= 1 << b;
                    node = want;
                } else {
                    node = child[2 * node + bit];
                }
            }
            best = max(best, cur);
        }
        return best;
    }
};
