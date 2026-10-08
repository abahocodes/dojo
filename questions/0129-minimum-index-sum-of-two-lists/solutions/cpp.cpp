class Solution {
public:
    vector<string> findRestaurant(vector<string>& list1, vector<string>& list2) {
        unordered_map<string, int> index;
        for (int i = 0; i < (int)list1.size(); i++) index[list1[i]] = i;
        int best = INT_MAX;
        vector<string> result;
        for (int j = 0; j < (int)list2.size(); j++) {
            auto it = index.find(list2[j]);
            if (it == index.end()) continue;
            int total = it->second + j;
            if (total < best) {
                best = total;
                result.assign(1, list2[j]);
            } else if (total == best) {
                result.push_back(list2[j]);
            }
        }
        return result;
    }
};
