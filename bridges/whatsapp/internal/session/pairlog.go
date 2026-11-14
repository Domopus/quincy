package session

import (
	"sync/atomic"

	waLog "go.mau.fi/whatsmeow/util/log"
)

// pairingLog prints whatsmeow's logs at DEBUG until pairing ends, then drops
// everything: once paired, messages flow and must never reach a log.
type pairingLog struct {
	waLog.Logger
	quiet *atomic.Bool
}

func newPairingLog() (waLog.Logger, *atomic.Bool) {
	quiet := &atomic.Bool{}
	return pairingLog{waLog.Stdout("whatsmeow", "DEBUG", true), quiet}, quiet
}

func (l pairingLog) Errorf(msg string, args ...any) {
	if !l.quiet.Load() {
		l.Logger.Errorf(msg, args...)
	}
}

func (l pairingLog) Warnf(msg string, args ...any) {
	if !l.quiet.Load() {
		l.Logger.Warnf(msg, args...)
	}
}

func (l pairingLog) Infof(msg string, args ...any) {
	if !l.quiet.Load() {
		l.Logger.Infof(msg, args...)
	}
}

func (l pairingLog) Debugf(msg string, args ...any) {
	if !l.quiet.Load() {
		l.Logger.Debugf(msg, args...)
	}
}

func (l pairingLog) Sub(module string) waLog.Logger {
	return pairingLog{l.Logger.Sub(module), l.quiet}
}
