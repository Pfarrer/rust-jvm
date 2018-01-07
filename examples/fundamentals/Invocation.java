package fundamentals;

public class Invocation {

  private int i;

  public Invocation() {
    this.i = 100;
  }

  public static void main(String[] args) {
    oneParam(1);
    twoParams(2, 3L);
    threeParams("three Params", 4, 5L);

    Invocation invocation = new Invocation();
    invocation.add(6, "Add 6");
    invocation.add(7, 8, "Add 8");
    call(invocation, 9);
  }

  public static int oneParam(int i1) {
    return i1;
  }

  public static long twoParams(int i1, long l1) {
    return i1 + l1;
  }

  public static long threeParams(String desc, int i1, long l1) {
    desc = null;
    return i1 + l1;
  }

  public static int call(Invocation invocation, int val) {
    return invocation.add(val, "call");
  }

  public int add(int v, String desc) {
    desc = null;
    return i + v;
  }

  public long add(int v, long a, String desc) {
    desc = null;
    return i + v + a;
  }
}
