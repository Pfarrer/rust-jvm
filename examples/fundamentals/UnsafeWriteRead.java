package fundamentals;

import sun.misc.Unsafe;
import java.lang.reflect.Field;
import java.lang.Exception;

public class UnsafeWriteRead {

  public static void main(String[] args) throws Exception {
    //Unsafe unsafe = Unsafe.getUnsafe();
    Field f = Unsafe.class.getDeclaredField("theUnsafe");
    f.setAccessible(true);
    Unsafe unsafe = (Unsafe) f.get(null);
 
    long m = unsafe.allocateMemory(8);
    unsafe.putLong(m, 0x0102030405060708L);

    byte b0 = unsafe.getByte(m);
    byte b2 = unsafe.getByte(m+2);
    byte b4 = unsafe.getByte(m+4);
    byte b7 = unsafe.getByte(m+7);

    System.out.println(
      "b0="+b0+
      " b2="+b2+
      " b4="+b4+
      " b7="+b7
    ); // Will print b0=8 b2=6 b4=4 b7=1

    unsafe.putLong(m, 72623859790382856L);
    System.out.println("b0="+unsafe.getByte(m));
  }

}
