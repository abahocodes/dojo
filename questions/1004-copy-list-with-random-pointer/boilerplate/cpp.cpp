// 1004. Copy List with Random Pointer (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Node {
public:
    int val;
    Node* next;
    Node* random;

    Node(int _val) {
        val = _val;
        next = nullptr;
        random = nullptr;
    }
};

class Solution {
public:
    Node* copyRandomList(Node* head) {
        return nullptr;
    }

    // dojo calls this to run the tests; you don't need to change it.
    vector<vector<int>> runCopyRandomList(vector<vector<int>>& nodes) {
        int n = nodes.size();
        vector<Node*> originals(n);
        for (int i = 0; i < n; i++) originals[i] = new Node(nodes[i][0]);
        auto nextOf = [&](int i) { return i + 1 < n ? originals[i + 1] : nullptr; };
        auto randomOf = [&](int r) { return r >= 0 ? originals[r] : nullptr; };
        for (int i = 0; i < n; i++) {
            originals[i]->next = nextOf(i);
            originals[i]->random = randomOf(nodes[i][1]);
        }
        Node* copy = copyRandomList(n > 0 ? originals[0] : nullptr);

        // The original list must be left as it was.
        for (int i = 0; i < n; i++) {
            Node* node = originals[i];
            if (node->val != nodes[i][0] || node->next != nextOf(i) || node->random != randomOf(nodes[i][1])) {
                return {{-1}};
            }
        }
        // Walk the copy: every node must be new, and the list must end.
        unordered_set<Node*> originalSet(originals.begin(), originals.end());
        unordered_map<Node*, int> index;
        vector<Node*> order;
        for (Node* node = copy; node != nullptr; node = node->next) {
            if (originalSet.count(node) || index.count(node)) return {{-1}};
            index[node] = order.size();
            order.push_back(node);
        }
        vector<vector<int>> result;
        for (Node* node : order) {
            if (node->random == nullptr) {
                result.push_back({node->val, -1});
            } else if (index.count(node->random)) {
                result.push_back({node->val, index[node->random]});
            } else {
                return {{-1}};
            }
        }
        return result;
    }
};
