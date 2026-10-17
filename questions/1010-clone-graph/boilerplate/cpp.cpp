// 1010. Clone Graph (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Node {
public:
    int val;
    vector<Node*> neighbors;

    Node() : val(0) {}
    Node(int _val) : val(_val) {}
    Node(int _val, vector<Node*> _neighbors) : val(_val), neighbors(_neighbors) {}
};

class Solution {
public:
    Node* cloneGraph(Node* node) {
        return nullptr;
    }

    // dojo calls this to run the tests; you don't need to change it.
    vector<vector<int>> runCloneGraph(vector<vector<int>>& adjList) {
        int n = adjList.size();
        vector<Node*> originals(n);
        for (int i = 0; i < n; i++) originals[i] = new Node(i + 1);
        for (int i = 0; i < n; i++) {
            for (int v : adjList[i]) originals[i]->neighbors.push_back(originals[v - 1]);
        }
        Node* clone = cloneGraph(n > 0 ? originals[0] : nullptr);

        // The original graph must be left as it was.
        for (int i = 0; i < n; i++) {
            Node* node = originals[i];
            if (node->val != i + 1 || node->neighbors.size() != adjList[i].size()) return {{-1}};
            for (size_t j = 0; j < adjList[i].size(); j++) {
                if (node->neighbors[j] != originals[adjList[i][j] - 1]) return {{-1}};
            }
        }
        if (clone == nullptr) return {};
        // Explore the clone: no original node, one node per value 1..n.
        unordered_set<Node*> originalSet(originals.begin(), originals.end());
        unordered_set<Node*> seen;
        vector<Node*> byVal(n + 1, nullptr);
        vector<Node*> stack{clone};
        while (!stack.empty()) {
            Node* node = stack.back();
            stack.pop_back();
            if (node == nullptr || originalSet.count(node)) return {{-1}};
            if (!seen.insert(node).second) continue;
            if (node->val < 1 || node->val > n || byVal[node->val] != nullptr) return {{-1}};
            byVal[node->val] = node;
            for (Node* nb : node->neighbors) stack.push_back(nb);
        }
        vector<vector<int>> result(n);
        for (int v = 1; v <= n; v++) {
            if (byVal[v] == nullptr) continue;
            for (Node* nb : byVal[v]->neighbors) result[v - 1].push_back(nb->val);
            sort(result[v - 1].begin(), result[v - 1].end());
        }
        return result;
    }
};
