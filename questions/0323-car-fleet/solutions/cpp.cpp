class Solution {
public:
    int carFleet(int target, vector<int>& position, vector<int>& speed) {
        int n = position.size();
        vector<pair<int, int>> cars(n);
        for (int i = 0; i < n; i++) cars[i] = {position[i], speed[i]};
        sort(cars.begin(), cars.end(), greater<pair<int, int>>());
        int fleets = 0;
        long long leadDist = 0, leadSpeed = 1;
        for (auto& [p, s] : cars) {
            long long dist = target - p;
            if (dist * leadSpeed > leadDist * s) {
                fleets++;
                leadDist = dist;
                leadSpeed = s;
            }
        }
        return fleets;
    }
};
