// Package gigasecond contains a function for adding a gigasecond to a time
package gigasecond

// import path for the time package from the standard library
import "time"

// AddGigasecond adds One Gigasecond, one billion seconds, to the input time
func AddGigasecond(t time.Time) time.Time {
	return t.Add(time.Second * 1e9)
}
