package fundamentals;

public class CharsetProperties {

  public static void main(String[] args) {
    System.out.println(System.getProperty("sun.stdout.encoding"));
    System.out.println(System.getProperty("sun.stderr.encoding"));
    System.out.println(System.getProperty("file.encoding"));
  }

}
