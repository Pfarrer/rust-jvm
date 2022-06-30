public interface InterfaceWithDefaultAndStatic {
    default double methodWithDefault(int num) {
        return 0.0 + num;
    }

    static String staticMethod() {
        return null;
    }
}