# Hints

## Hint 1
Keep, for every user, the set of people they follow and the list of their own
tweets in posting order. A global counter that increases on every post gives
each tweet a timestamp, so tweets from different users can be compared.

## Hint 2
Sorting every tweet of every followed user on each `getNewsFeed` is wasteful.
Each user's tweet list is already sorted by time, and you only need the top
10 overall. Only the last 10 tweets of each user can possibly make the feed.

## Hint 3
Merge the per-user lists like in "merge k sorted lists": push the newest
tweet of each relevant user (the requester plus their followees) onto a
max-heap keyed by timestamp. Pop 10 times; after popping a tweet, push the
next older tweet from the same user.
