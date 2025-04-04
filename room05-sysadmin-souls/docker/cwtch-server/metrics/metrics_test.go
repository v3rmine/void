package metrics

import (
	"testing"
	"time"
)

func TestCounter(t *testing.T) {
	starttime := time.Now()
	c := NewCounter()

	max := 100
	done := make(chan bool, max)

	// slightly stress test atomic nature of metric by flooding with threads Add()ing
	for i := 0; i < max; i++ {
		go func() {
			c.Add(1)
			done <- true
		}()
	}

	for i := 0; i < max; i++ {
		<-done
	}

	val := c.Count()
	if val != 100 {
		t.Errorf("counter count was not 100")
	}

	counterStart := c.GetStarttime()

	if counterStart.Sub(starttime) > time.Millisecond {
		t.Errorf("counter's starttime was innaccurate %v", counterStart.Sub(starttime))
	}
}
