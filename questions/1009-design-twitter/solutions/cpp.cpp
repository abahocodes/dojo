class Twitter {
    int time = 0;
    unordered_map<int, vector<pair<int, int>>> tweets;  // user -> (time, tweetId) in posting order
    unordered_map<int, unordered_set<int>> follows;     // user -> followees (never the user itself)

public:
    Twitter() {}

    void postTweet(int userId, int tweetId) {
        tweets[userId].push_back({time++, tweetId});
    }

    vector<int> getNewsFeed(int userId) {
        // Max-heap by time over (time, user, index): the newest unread tweet per user.
        priority_queue<tuple<int, int, int>> heap;
        auto seed = [&](int u) {
            auto it = tweets.find(u);
            if (it == tweets.end() || it->second.empty()) return;
            int i = (int)it->second.size() - 1;
            heap.push({it->second[i].first, u, i});
        };
        seed(userId);
        auto f = follows.find(userId);
        if (f != follows.end()) {
            for (int u : f->second) seed(u);
        }
        vector<int> feed;
        while (!heap.empty() && feed.size() < 10) {
            auto [t, u, i] = heap.top();
            heap.pop();
            const auto& list = tweets[u];
            feed.push_back(list[i].second);
            if (i > 0) heap.push({list[i - 1].first, u, i - 1});
        }
        return feed;
    }

    void follow(int followerId, int followeeId) {
        if (followerId != followeeId) follows[followerId].insert(followeeId);
    }

    void unfollow(int followerId, int followeeId) {
        auto f = follows.find(followerId);
        if (f != follows.end()) f->second.erase(followeeId);
    }
};

// dojo calls this to run the tests; you don't need to change it.
class Solution {
public:
    vector<vector<int>> runTwitter(vector<string>& ops, vector<vector<int>>& args) {
        Twitter twitter;
        vector<vector<int>> out;
        for (size_t i = 0; i < ops.size(); i++) {
            const vector<int>& a = args[i];
            if (ops[i] == "postTweet") twitter.postTweet(a[0], a[1]);
            else if (ops[i] == "getNewsFeed") out.push_back(twitter.getNewsFeed(a[0]));
            else if (ops[i] == "follow") twitter.follow(a[0], a[1]);
            else twitter.unfollow(a[0], a[1]);
        }
        return out;
    }
};
