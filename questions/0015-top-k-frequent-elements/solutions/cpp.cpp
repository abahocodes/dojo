class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        unordered_map<int, int> counts;
        for (int x : nums) counts[x]++;

        // buckets[f] holds every value that occurs exactly f times
        vector<vector<int>> buckets(nums.size() + 1);
        for (auto& [value, freq] : counts) buckets[freq].push_back(value);

        vector<int> result;
        for (int freq = (int)nums.size(); freq > 0; freq--) {
            for (int value : buckets[freq]) {
                result.push_back(value);
                if ((int)result.size() == k) return result;
            }
        }
        return result;
    }
};
