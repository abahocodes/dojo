# 1009. Design Twitter (medium)
# /show to reread the problem · /test runs visible tests · /submit when done

class Twitter:
    def __init__(self):
        pass

    def postTweet(self, userId: int, tweetId: int) -> None:
        pass

    def getNewsFeed(self, userId: int) -> list[int]:
        return []

    def follow(self, followerId: int, followeeId: int) -> None:
        pass

    def unfollow(self, followerId: int, followeeId: int) -> None:
        pass


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
