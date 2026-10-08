class Solution {
public:
    vector<int> frequencySortNumbers(vector<int>& nums) {
        int count[201] = {0};
        for (int x : nums) count[x + 100]++;
        vector<int> result = nums;
        sort(result.begin(), result.end(), [&](int a, int b) {
            if (count[a + 100] != count[b + 100]) return count[a + 100] < count[b + 100];
            return a > b;
        });
        return result;
    }
};
