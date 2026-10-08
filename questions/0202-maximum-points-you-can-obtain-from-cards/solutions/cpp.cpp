class Solution {
public:
    int maxScoreCards(vector<int>& cardPoints, int k) {
        int n = cardPoints.size();
        int current = 0;
        for (int i = 0; i < k; i++) current += cardPoints[i];
        int best = current;
        for (int i = 1; i <= k; i++) {
            current += cardPoints[n - i] - cardPoints[k - i];
            best = max(best, current);
        }
        return best;
    }
};
