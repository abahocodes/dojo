package main

import "sort"

type tweet struct {
	time, id int
}

type Twitter struct {
	time    int
	tweets  map[int][]tweet           // user -> tweets in posting order
	follows map[int]map[int]struct{} // user -> followees (never the user itself)
}

func Constructor() Twitter {
	return Twitter{tweets: map[int][]tweet{}, follows: map[int]map[int]struct{}{}}
}

func (t *Twitter) PostTweet(userId int, tweetId int) {
	t.tweets[userId] = append(t.tweets[userId], tweet{t.time, tweetId})
	t.time++
}

func (t *Twitter) GetNewsFeed(userId int) []int {
	// Only the last 10 tweets of each relevant user can make the feed.
	var pool []tweet
	take := func(u int) {
		list := t.tweets[u]
		start := len(list) - 10
		if start < 0 {
			start = 0
		}
		pool = append(pool, list[start:]...)
	}
	take(userId)
	for u := range t.follows[userId] {
		take(u)
	}
	sort.Slice(pool, func(i, j int) bool { return pool[i].time > pool[j].time })
	feed := []int{}
	for i := 0; i < len(pool) && i < 10; i++ {
		feed = append(feed, pool[i].id)
	}
	return feed
}

func (t *Twitter) Follow(followerId int, followeeId int) {
	if followerId == followeeId {
		return
	}
	if t.follows[followerId] == nil {
		t.follows[followerId] = map[int]struct{}{}
	}
	t.follows[followerId][followeeId] = struct{}{}
}

func (t *Twitter) Unfollow(followerId int, followeeId int) {
	delete(t.follows[followerId], followeeId)
}

// dojo calls this to run the tests; you don't need to change it.
func runTwitter(ops []string, args [][]int) [][]int {
	twitter := Constructor()
	out := [][]int{}
	for i, op := range ops {
		a := args[i]
		switch op {
		case "postTweet":
			twitter.PostTweet(a[0], a[1])
		case "getNewsFeed":
			out = append(out, append([]int{}, twitter.GetNewsFeed(a[0])...))
		case "follow":
			twitter.Follow(a[0], a[1])
		default:
			twitter.Unfollow(a[0], a[1])
		}
	}
	return out
}
