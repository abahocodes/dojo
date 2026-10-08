class Solution {
public:
    bool canFinish(int numCourses, vector<vector<int>>& prerequisites) {
        vector<vector<int>> unlocks(numCourses);
        vector<int> indegree(numCourses, 0);
        for (auto& p : prerequisites) {
            unlocks[p[1]].push_back(p[0]);
            indegree[p[0]]++;
        }

        queue<int> ready;
        for (int i = 0; i < numCourses; i++) if (indegree[i] == 0) ready.push(i);
        int taken = 0;
        while (!ready.empty()) {
            int current = ready.front();
            ready.pop();
            taken++;
            for (int nxt : unlocks[current]) {
                if (--indegree[nxt] == 0) ready.push(nxt);
            }
        }
        return taken == numCourses;
    }
};
