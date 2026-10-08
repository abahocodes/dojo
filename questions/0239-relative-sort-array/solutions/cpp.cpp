class Solution {
public:
    vector<int> relativeSortArray(vector<int>& arr1, vector<int>& arr2) {
        vector<int> count(1001, 0);
        for (int x : arr1) count[x]++;
        vector<int> result;
        result.reserve(arr1.size());
        for (int x : arr2) {
            result.insert(result.end(), count[x], x);
            count[x] = 0;
        }
        for (int x = 0; x <= 1000; x++) {
            result.insert(result.end(), count[x], x);
        }
        return result;
    }
};
