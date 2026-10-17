// 1009. Design Twitter (medium)
// /show to reread the problem · /test runs visible tests · /submit when done
package main

type Twitter struct {
}

func Constructor() Twitter {
	return Twitter{}
}

func (t *Twitter) PostTweet(userId int, tweetId int) {

}

func (t *Twitter) GetNewsFeed(userId int) []int {
	return nil
}

func (t *Twitter) Follow(followerId int, followeeId int) {

}

func (t *Twitter) Unfollow(followerId int, followeeId int) {

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
