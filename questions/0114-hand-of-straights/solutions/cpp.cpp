class Solution {
public:
    bool isNStraightHand(vector<int>& hand, int groupSize) {
        if (hand.size() % groupSize != 0) return false;
        map<int, int> count;
        for (int x : hand) count[x]++;
        for (auto& [x, c] : count) {
            int need = c;
            if (need == 0) continue;
            for (int v = x; v < x + groupSize; v++) {
                auto it = count.find(v);
                if (it == count.end() || it->second < need) return false;
                it->second -= need;
            }
        }
        return true;
    }
};
