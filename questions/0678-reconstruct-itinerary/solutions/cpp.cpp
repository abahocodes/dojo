class Solution {
public:
    vector<string> findItinerary(vector<vector<string>>& tickets) {
        // Destinations sorted descending, so pop_back() hands out the smallest first.
        unordered_map<string, vector<string>> graph;
        for (auto& t : tickets) graph[t[0]].push_back(t[1]);
        for (auto& [src, out] : graph) sort(out.rbegin(), out.rend());

        // Hierholzer: walk greedily; an airport with no tickets left is final,
        // so it goes onto the route (built back to front).
        vector<string> route;
        vector<string> stack = {"JFK"};
        while (!stack.empty()) {
            auto it = graph.find(stack.back());
            if (it != graph.end() && !it->second.empty()) {
                string next = it->second.back();
                it->second.pop_back();
                stack.push_back(next);
            } else {
                route.push_back(stack.back());
                stack.pop_back();
            }
        }
        reverse(route.begin(), route.end());
        return route;
    }
};
