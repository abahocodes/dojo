class Solution {
public:
    vector<int> findOrder(int numCourses, vector<vector<int>>& prerequisites) {
        vector<vector<int>> unlocks(numCourses);
        vector<int> indegree(numCourses, 0);
        for (auto& p : prerequisites) {
            unlocks[p[1]].push_back(p[0]);
            indegree[p[0]]++;
        }

        // A min-heap always hands out the smallest course that is ready now,
        // which gives the lexicographically smallest valid order.
        priority_queue<int, vector<int>, greater<int>> ready;
        for (int i = 0; i < numCourses; i++) if (indegree[i] == 0) ready.push(i);
        vector<int> order;
        while (!ready.empty()) {
            int current = ready.top();
            ready.pop();
            order.push_back(current);
            for (int next : unlocks[current]) {
                if (--indegree[next] == 0) ready.push(next);
            }
        }
        return (int)order.size() == numCourses ? order : vector<int>{};
    }
};
