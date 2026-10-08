class Solution {
public:
    TreeNode* deserialize(string& data) {
        vector<string> tokens;
        stringstream ss(data);
        string tok;
        while (getline(ss, tok, ',')) tokens.push_back(tok);
        if (tokens.empty() || tokens[0] == "#") return nullptr;
        TreeNode* root = new TreeNode(stoi(tokens[0]));
        vector<TreeNode*> stack{root};
        vector<bool> leftDone{false};
        for (size_t i = 1; i < tokens.size(); i++) {
            TreeNode* child = tokens[i] == "#" ? nullptr : new TreeNode(stoi(tokens[i]));
            TreeNode* parent = stack.back();
            if (!leftDone.back()) {
                parent->left = child;
                leftDone.back() = true;
            } else {
                parent->right = child;
                stack.pop_back();
                leftDone.pop_back();
            }
            if (child != nullptr) {
                stack.push_back(child);
                leftDone.push_back(false);
            }
        }
        return root;
    }
};
