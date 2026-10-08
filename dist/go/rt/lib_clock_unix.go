package rt

import nativeclock "goalchemyout/cap/clock"

func LibClockUnix() int64 { return nativeclock.Unix() }
