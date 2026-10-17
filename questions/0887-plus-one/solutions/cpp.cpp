class Solution {
public:
    vector<int> plusOne(vector<int>& digits) {
        // Walk from the last digit: a 9 becomes 0 and carries; anything else
        // just increments and we're done.
        vector<int> result = digits;
        for (int i = (int)result.size() - 1; i >= 0; i--) {
            if (result[i] < 9) {
                result[i]++;
                return result;
            }
            result[i] = 0;
        }
        // Every digit was 9: the answer is 1 followed by zeros.
        result.insert(result.begin(), 1);
        return result;
    }
};
