Build the core of a tiny social network. Users post short messages
("tweets"), follow each other, and ask for a news feed of the most recent
tweets from themselves and the people they follow.

Implement the class `Twitter`:

- `Twitter()` creates an empty network: no users follow anyone and no tweets
  exist.
- `postTweet(userId, tweetId)` records that user `userId` posted a new tweet
  with id `tweetId`. Every call posts a later tweet than all earlier calls,
  and tweet ids are never reused.
- `getNewsFeed(userId)` returns the ids of the **10 most recent** tweets
  posted by `userId` or by anyone `userId` currently follows, newest first.
  If there are fewer than 10 such tweets, return all of them.
- `follow(followerId, followeeId)` makes `followerId` follow `followeeId`.
  Following someone you already follow, or yourself, changes nothing.
- `unfollow(followerId, followeeId)` makes `followerId` stop following
  `followeeId`. Unfollowing someone you don't follow, or yourself, changes
  nothing: a user always sees their own tweets.

Following takes effect for old tweets too: right after `a` follows `b`, all
of `b`'s earlier tweets can appear in `a`'s feed, and after an unfollow they
disappear again.

## How the tests call your class

Each test is a sequence of operations on one fresh `Twitter`. `ops[i]` names
the method and `args[i]` holds its integer arguments in order:

| `ops[i]`        | `args[i]`                    |
|-----------------|------------------------------|
| `"postTweet"`   | `[userId, tweetId]`          |
| `"getNewsFeed"` | `[userId]`                   |
| `"follow"`      | `[followerId, followeeId]`   |
| `"unfollow"`    | `[followerId, followeeId]`   |

The driver at the bottom of the starter code runs them in order and returns
the list of feeds, one per `getNewsFeed` call; the other methods return
nothing and add nothing to the output. You only need to fill in the class.

## Example 1

```
ops    = ["postTweet", "postTweet", "getNewsFeed", "follow", "postTweet", "getNewsFeed", "unfollow", "getNewsFeed"]
args   = [[1, 11], [2, 20], [1], [1, 2], [2, 21], [1], [1, 2], [1]]
output = [[11], [21, 20, 11], [11]]
```

At first user `1` sees only tweet `11`. After following user `2`, the feed
holds `2`'s old tweet `20` and new tweet `21`, newest first. Unfollowing
hides them again.

## Example 2

```
ops    = ["getNewsFeed", "follow", "follow", "postTweet", "unfollow", "getNewsFeed"]
args   = [[7], [7, 7], [3, 7], [7, 5], [7, 7], [3]]
output = [[], [5]]
```

A user with no tweets and no followees has an empty feed. Following and
unfollowing yourself does nothing.

## Constraints

- `1 <= len(ops) == len(args) <= 3 * 10^4`
- `1 <= userId, followerId, followeeId <= 500`
- `0 <= tweetId <= 10^5`, and all tweet ids are distinct.
- At most `3 * 10^4` calls in total.
