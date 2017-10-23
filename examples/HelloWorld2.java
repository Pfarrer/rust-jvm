package hello.world;

import java.io.Serializable;

public class HelloWorld2 implements Serializable {

  private final static String MSG = "Hello World!";

  private final String msg;

  public HelloWorld2(String msg) {
    this.msg = msg;
  }

  public void greet() {
    System.out.println(msg);
  }

  public static void main(String[] args) {
    new HelloWorld2(MSG).greet();
  }
}
