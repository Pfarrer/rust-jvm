package fundamentals;

public class Basic2 {

  public static void main(String[] args) {
    int value1 = 1;
    int value10 = 1;
    int value2 = 2;

    if (value1 == value10) eq();
    else NOOOO();

    if (value1 != value2) ne();
    else NOOOO();

    if (value1 < value2) lt();
    else NOOOO();

    if (value1 <= value2) le();
    else NOOOO();

    if (value2 > value1) gt();
    else NOOOO();

    if (value2 >= value1) ge();
    else NOOOO();
  }

  public static void eq() {}
  public static void ne() {}
  public static void lt() {}
  public static void le() {}
  public static void gt() {}
  public static void ge() {}

  public static void NOOOO() {}
}
