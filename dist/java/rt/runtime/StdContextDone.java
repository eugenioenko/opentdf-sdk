package io.goalchemy.runtime;

/** std.context.done: the channel closed on cancellation. */
public final class StdContextDone {
  private StdContextDone() {}

  public static ChanMake.Chan stdContextContextDone(StdContextErr.Context c) {
    StdContextErr.observe(c);
    return c.done;
  }
}
