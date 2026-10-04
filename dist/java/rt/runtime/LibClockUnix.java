package io.goalchemy.runtime;

public final class LibClockUnix {
  private LibClockUnix() {}

  public static long libClockUnix() {
    return java.time.Instant.now().getEpochSecond();
  }
}
