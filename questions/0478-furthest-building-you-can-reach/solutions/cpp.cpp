class Solution {
public:
    int furthestBuilding(vector<int>& heights, int bricks, int ladders) {
        priority_queue<int, vector<int>, greater<int>> ladderClimbs;
        for (int i = 0; i + 1 < (int)heights.size(); i++) {
            int climb = heights[i + 1] - heights[i];
            if (climb <= 0) continue;
            ladderClimbs.push(climb);
            if ((int)ladderClimbs.size() > ladders) {
                bricks -= ladderClimbs.top();
                ladderClimbs.pop();
                if (bricks < 0) return i;
            }
        }
        return (int)heights.size() - 1;
    }
};
