class DetectSquares {
    // Coordinates are 0..1000, so a grid of counts is small enough.
    vector<vector<int>> cnt = vector<vector<int>>(1001, vector<int>(1001, 0));
    vector<vector<int>> ys = vector<vector<int>>(1001);  // x -> distinct ys in that column

public:
    DetectSquares() {}

    void add(vector<int> point) {
        int x = point[0], y = point[1];
        if (cnt[x][y]++ == 0) ys[x].push_back(y);
    }

    int count(vector<int> point) {
        int qx = point[0], qy = point[1];
        long long total = 0;
        // Every point in the query's column fixes the side length.
        for (int y : ys[qx]) {
            int side = y - qy;
            if (side == 0) continue;
            long long c = cnt[qx][y];
            for (int px : {qx + side, qx - side}) {
                if (px < 0 || px > 1000) continue;
                total += c * cnt[px][qy] * cnt[px][y];
            }
        }
        return (int)total;
    }
};

// dojo calls this to run the tests; you don't need to change it.
class Solution {
public:
    vector<int> runDetectSquares(vector<string>& ops, vector<vector<int>>& points) {
        DetectSquares ds;
        vector<int> out;
        for (size_t i = 0; i < ops.size(); i++) {
            if (ops[i] == "add") ds.add(points[i]);
            else out.push_back(ds.count(points[i]));
        }
        return out;
    }
};
