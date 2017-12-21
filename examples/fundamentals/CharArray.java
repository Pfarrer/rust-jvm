package fundamentals;

public class CharArray {

  public static void main(String[] args) {
    char[] arr = new char[10];
    arr[0] = 'a';

    for (int i = 1; i < arr.length; i++) {
      arr[i] = arr[i-1];
    }
  }

}
