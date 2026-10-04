package io.goalchemy.runtime;

/** std.context.background: the never-cancelled root context. */
public final class StdContextBackground {
  private StdContextBackground() {}

  public static StdContextErr.Context stdContextBackground() {
    return StdContextErr.BACKGROUND;
  }
}
