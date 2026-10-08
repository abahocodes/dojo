class Solution {
public:
    string orderlyQueue(string& s, int k) {
        if (k == 1) {
            string doubled = s + s;
            string best = s;
            for (size_t i = 1; i < s.size(); i++) {
                if (doubled.compare(i, s.size(), best) < 0) best = doubled.substr(i, s.size());
            }
            return best;
        }
        string sorted = s;
        sort(sorted.begin(), sorted.end());
        return sorted;
    }
};
