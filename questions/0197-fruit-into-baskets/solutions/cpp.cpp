class Solution {
public:
    int totalFruit(vector<int>& fruits) {
        unordered_map<int, int> count;
        int left = 0, best = 0;
        for (int right = 0; right < (int)fruits.size(); right++) {
            count[fruits[right]]++;
            while (count.size() > 2) {
                int g = fruits[left++];
                if (--count[g] == 0) count.erase(g);
            }
            best = max(best, right - left + 1);
        }
        return best;
    }
};
