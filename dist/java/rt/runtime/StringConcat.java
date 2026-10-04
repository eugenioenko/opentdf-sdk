package io.goalchemy.runtime;

/** core.string.concat: byte concatenation. */
public final class StringConcat {
  private StringConcat() {}

  public static String concat(String a, String b) {
    return a + b;
  }
}
