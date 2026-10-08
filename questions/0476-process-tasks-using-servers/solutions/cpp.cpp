class Solution {
public:
    vector<int> assignTasks(vector<int>& servers, vector<int>& tasks) {
        using Free = pair<int, int>;                  // weight, index
        using Busy = tuple<long long, int, int>;      // free time, weight, index
        priority_queue<Free, vector<Free>, greater<Free>> freeQ;
        priority_queue<Busy, vector<Busy>, greater<Busy>> busy;
        for (int i = 0; i < (int)servers.size(); i++) freeQ.push({servers[i], i});
        vector<int> result;
        result.reserve(tasks.size());
        long long time = 0;
        for (int j = 0; j < (int)tasks.size(); j++) {
            time = max(time, (long long)j);
            if (freeQ.empty()) time = max(time, get<0>(busy.top()));
            while (!busy.empty() && get<0>(busy.top()) <= time) {
                auto [t, w, i] = busy.top();
                busy.pop();
                freeQ.push({w, i});
            }
            auto [w, i] = freeQ.top();
            freeQ.pop();
            result.push_back(i);
            busy.push({time + tasks[j], w, i});
        }
        return result;
    }
};
