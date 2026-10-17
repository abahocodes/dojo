import heapq
from collections import defaultdict


class Twitter:
    def __init__(self):
        self.time = 0
        self.tweets = defaultdict(list)  # user -> [(time, tweet_id)] in posting order
        self.follows = defaultdict(set)  # user -> followees (never the user itself)

    def postTweet(self, userId: int, tweetId: int) -> None:
        self.tweets[userId].append((self.time, tweetId))
        self.time += 1

    def getNewsFeed(self, userId: int) -> list[int]:
        # Max-heap (by time) over the newest unread tweet of each relevant user.
        heap = []
        for u in self.follows[userId] | {userId}:
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

    def follow(self, followerId: int, followeeId: int) -> None:
        if followerId != followeeId:
            self.follows[followerId].add(followeeId)

    def unfollow(self, followerId: int, followeeId: int) -> None:
        self.follows[followerId].discard(followeeId)


# dojo calls this to run the tests; you don't need to change it.
def run_twitter(ops: list[str], args: list[list[int]]) -> list[list[int]]:
    twitter = Twitter()
    out = []
    for op, a in zip(ops, args):
        if op == "postTweet":
            twitter.postTweet(a[0], a[1])
        elif op == "getNewsFeed":
            out.append(list(twitter.getNewsFeed(a[0])))
        elif op == "follow":
            twitter.follow(a[0], a[1])
        else:
            twitter.unfollow(a[0], a[1])
    return out
