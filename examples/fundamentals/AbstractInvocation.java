package fundamentals;

public abstract class AbstractInvocation {

  public static void main(String[] args) {
    Impl impl = new Impl();

    testDirect(impl);
    testAbstract(impl);

    Impl.staticMethod();
    impl.hashCode();
  }

  static void testDirect(Impl impl) {
    impl.method('d', 1);
  }

  static void testAbstract(AbstractInvocation instance) {
    instance.method('a', 2);
  }

  public abstract int method(char c, int i);

  public static void staticMethod() {

  }

  static class Impl extends AbstractInvocation {

    public int method(char c, int i) {
      return i + (int) c;
    }

  }

}
