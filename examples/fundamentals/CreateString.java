package fundamentals;

public class CreateString {

  public static void main(String[] args) {
    String staticString = "Hello World! ";

    String dynamicString = new String();
    for (int i=0; i<5; i++) {
      dynamicString = dynamicString + staticString;
    }

    dynamicString.trim();
  }

}
