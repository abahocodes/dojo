class Solution {
public:
    vector<int> findOriginalArray(vector<int>& changed) {
        if (changed.size() % 2 != 0) return {};
        int top = *max_element(changed.begin(), changed.end());
        vector<int> count(top + 1, 0);
        for (int v : changed) count[v]++;
        if (count[0] % 2 != 0) return {};
        vector<int> original(count[0] / 2, 0);
        for (int x = 1; x <= top; x++) {
            int c = count[x];
            if (c == 0) continue;
            if (2 * x > top || count[2 * x] < c) return {};
            count[2 * x] -= c;
            original.insert(original.end(), c, x);
        }
        return original;
    }
};
