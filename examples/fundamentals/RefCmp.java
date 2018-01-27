package fundamentals;

public class RefCmp {

  public static void main(String[] args) {
    String a = new String("abc");
    String b = new String("def");

    if (a == b) FAIL();
    else correct();

    if (a != b) correct();
    else FAIL();

    if (a == a) correct();
    else FAIL();

    if (a != a) FAIL();
    else correct();
  }

  public static void correct() {
  }

  public static void FAIL() {
  }

}
