// 1012. Detect Squares (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class DetectSquares {
public:
    DetectSquares() {

    }

    void add(vector<int> point) {

    }

    int count(vector<int> point) {
        return 0;
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
