class Solution {
    vector<int> parent;

    int find(int x) {
        while (parent[x] != x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    }

public:
    vector<vector<string>> accountsMerge(vector<vector<string>>& accounts) {
        int n = accounts.size();
        parent.resize(n);
        for (int i = 0; i < n; i++) parent[i] = i;

        unordered_map<string, int> owner;  // email -> index of the first account that listed it
        for (int i = 0; i < n; i++) {
            for (size_t k = 1; k < accounts[i].size(); k++) {
                const string& email = accounts[i][k];
                auto it = owner.find(email);
                if (it != owner.end()) {
                    int ri = find(i), rj = find(it->second);
                    if (ri != rj) parent[ri] = rj;
                } else {
                    owner.emplace(email, i);
                }
            }
        }

        unordered_map<int, vector<string>> groups;
        for (const auto& [email, i] : owner) groups[find(i)].push_back(email);

        vector<vector<string>> merged;
        for (auto& [root, emails] : groups) {
            sort(emails.begin(), emails.end());
            vector<string> account{accounts[root][0]};
            account.insert(account.end(), emails.begin(), emails.end());
            merged.push_back(move(account));
        }
        sort(merged.begin(), merged.end(), [](const vector<string>& a, const vector<string>& b) {
            return tie(a[0], a[1]) < tie(b[0], b[1]);
        });
        return merged;
    }
};
