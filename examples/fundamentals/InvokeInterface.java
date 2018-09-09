package fundamentals;

public class InvokeInterface {

    public static void main(String[] args) {
        MyInterface myInterface = new MyImplementation();
        invoke(myInterface);

        MyInterface myInterface2 = new AnotherSubclass();
        invoke(myInterface2);

        MyInterface myInterface3 = new InterfaceImpl();
        invoke(myInterface3);
    }

    private static void invoke(MyInterface myInterface) {
        myInterface.myMethodA();
        myInterface.myMethodB(123);
    }

    static interface MyInterface {
        int myMethodA();

        int myMethodB(int i);
    }

    static abstract class MyAbstractImplementation implements MyInterface {
        public int myMethodA() {
            return 11111;
        }

        public int myMethodB(int i) {
            return 22222 + i;
        }
    }

    static class MyImplementation extends MyAbstractImplementation {
    }

    static class AnotherSubclass extends MyImplementation {
        public int myMethodB(int i) {
            return i + 33333;
        }
    }

    static class InterfaceImpl implements MyInterface {
        public int myMethodA() {
            return 4444;
        }

        public int myMethodB(int i) {
            return 5555 + i;
        }
    }

}
