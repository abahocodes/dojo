# Approach: per-user timelines merged with a heap

State:

- `tweets[u]`: user `u`'s tweets as `(time, tweetId)` pairs in posting
  order. A global counter supplies `time`, so later tweets have larger times.
- `follows[u]`: the set of users `u` follows (never including `u` itself).

`postTweet`, `follow` and `unfollow` are O(1) updates (ignore self-follows
and self-unfollows). `getNewsFeed(u)` is a k-way merge of the sorted lists of
`u` and everyone in `follows[u]`, stopped after 10 items: seed a max-heap
with the newest tweet of each list, pop the newest overall, and replace it
with the next older tweet from the same list.

```python
import heapq
from collections import defaultdict

class Twitter:
    def __init__(self):
        self.time = 0
        self.tweets = defaultdict(list)    # user -> [(time, tweet_id)]
        self.follows = defaultdict(set)    # user -> followees

    def postTweet(self, user_id, tweet_id):
        self.tweets[user_id].append((self.time, tweet_id))
        self.time += 1

    def getNewsFeed(self, user_id):
        heap = []
        for u in self.follows[user_id] | {user_id}:
            if self.tweets[u]:
                i = len(self.tweets[u]) - 1
                t, tid = self.tweets[u][i]
                heap.append((-t, tid, u, i))
        heapq.heapify(heap)
        feed = []
        while heap and len(feed) < 10:
            _, tid, u, i = heapq.heappop(heap)
            feed.append(tid)
            if i > 0:
                t, nid = self.tweets[u][i - 1]
                heapq.heappush(heap, (-t, nid, u, i - 1))
        return feed

    def follow(self, a, b):
        if a != b:
            self.follows[a].add(b)

    def unfollow(self, a, b):
        self.follows[a].discard(b)
```

A simpler variant with the same bound: take the last 10 tweets of each
relevant user, sort that small pool by time and keep the first 10. It costs
O(10F log(10F)) instead of O(F + 10 log F) for `F` followees, which is fine
at these sizes.

## Complexity

- `postTweet`, `follow`, `unfollow`: O(1).
- `getNewsFeed`: O(F + 10 log F), where `F` is the number of followees.
- Space: O(total tweets + total follow relations).

## Pitfalls

- Forgetting the user's own tweets in their feed, or letting a
  self-unfollow remove them. Treat the user as always included, separately
  from the follow set.
- Comparing tweet ids instead of post times: ids are arbitrary and say
  nothing about order.
- Storing follows as a list lets duplicate follows sneak in, which can put
  the same tweet into the feed twice.
- Building each feed from all tweets of everyone followed is O(total tweets)
  per call, too slow with many posts and feed requests.
