package fundamentals;

import java.lang.RuntimePermission;

public class Dup2x1 {

  private static final long serialVersionUID = 6279438298436773498L;

  private transient boolean wildcard;

  private transient String path;

  private transient boolean exitVM;

  /**
   * initialize a BasicPermission object. Common to all constructors.
   */
  private void init(String name) {
    if (name == null)
      throw new NullPointerException("name can't be null");

    int len = name.length();

    if (len == 0) {
      throw new IllegalArgumentException("name can't be empty");
    }

    char last = name.charAt(len - 1);

    // Is wildcard or ends with ".*"?
    if (last == '*' && (len == 1 || name.charAt(len - 2) == '.')) {
      wildcard = true;
      if (len == 1) {
        path = "";
      } else {
        path = name.substring(0, len - 1);
      }
    } else {
      if (name.equals("exitVM")) {
        wildcard = true;
        path = "exitVM.";
        exitVM = true;
      } else {
        path = name;
      }
    }
  }

  public Dup2x1(String name) {
//    super(name);
    init(name);
  }

  public static void main(String[] args) {
//    new Dup2x1("myperm.*");
    new RuntimePermission("myperm.*");
  }

}
