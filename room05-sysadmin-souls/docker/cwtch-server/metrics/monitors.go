package metrics

import (
	"bufio"
	"fmt"
	"git.openprivacy.ca/cwtch.im/tapir"
	"git.openprivacy.ca/openprivacy/log"
	"os"
	"path"
	"runtime"
	"sync"
	"time"
)

const (
	reportFile = "serverMonitorReport.txt"
)

type MessageCountFn func() int

// Monitors is a package of metrics for a Cwtch Server including message count, CPU, Mem, and conns
type Monitors struct {
	MessageCounter Counter
	Messages       MonitorHistory
	Memory         MonitorHistory
	ClientConns    MonitorHistory
	messageCountFn MessageCountFn
	starttime      time.Time
	breakChannel   chan bool
	log            bool
	configDir      string
	running        bool
	lock           sync.Mutex
}

func bToMb(b uint64) uint64 {
	return b / 1024 / 1024
}

// Start initializes a Monitors's monitors
func (mp *Monitors) Start(ts tapir.Service, mcfn MessageCountFn, configDir string, doLogging bool) {
	mp.log = doLogging
	mp.configDir = configDir
	mp.starttime = time.Now()
	mp.breakChannel = make(chan bool)
	mp.MessageCounter = NewCounter()
	mp.messageCountFn = mcfn

	mp.Messages = NewMonitorHistory(Count, Cumulative, func() (c float64) {
		c = float64(mp.MessageCounter.Count())
		mp.MessageCounter.Reset()
		return
	})

	mp.Memory = NewMonitorHistory(MegaBytes, Average, func() float64 {
		var m runtime.MemStats
		runtime.ReadMemStats(&m)
		return float64(bToMb(m.Sys))
	})

	mp.ClientConns = NewMonitorHistory(Count, Average, func() float64 { return float64(ts.Metrics().ConnectionCount) })

	if mp.log {
		go mp.run()
	}
}

func (mp *Monitors) run() {
	mp.running = true
	for {
		select {
		case <-time.After(time.Minute):
			mp.lock.Lock()
			mp.report()
			mp.lock.Unlock()
		case <-mp.breakChannel:
			mp.lock.Lock()
			mp.running = false
			mp.lock.Unlock()
			return
		}
	}
}

func FormatDuration(ts time.Duration) string {
	const (
		Day = 24 * time.Hour
	)
	d := ts / Day
	ts = ts % Day
	h := ts / time.Hour
	ts = ts % time.Hour
	m := ts / time.Minute
	return fmt.Sprintf("%dd%dh%dm", d, h, m)
}

func (mp *Monitors) report() {
	f, err := os.Create(path.Join(mp.configDir, reportFile))
	if err != nil {
		log.Errorf("Could not open monitor reporting file: %v", err)
		return
	}
	defer f.Close()

	w := bufio.NewWriter(f)

	fmt.Fprintf(w, "Uptime: %v \n", FormatDuration(time.Since(mp.starttime)))
	fmt.Fprintf(w, "Total Messages: %v \n\n", mp.messageCountFn())

	fmt.Fprintln(w, "Messages:")
	mp.Messages.Report(w)

	fmt.Fprintln(w, "\nClient Connections:")
	mp.ClientConns.Report(w)

	fmt.Fprintln(w, "\nSys Memory:")
	mp.Memory.Report(w)

	w.Flush()
}

// Stop stops all the monitors in a Monitors
func (mp *Monitors) Stop() {
	mp.lock.Lock()
	running := mp.running
	mp.lock.Unlock()
	if running {
		if mp.log {
			mp.breakChannel <- true
		}
		mp.Messages.Stop()
		mp.Memory.Stop()
		mp.ClientConns.Stop()
	}
}
