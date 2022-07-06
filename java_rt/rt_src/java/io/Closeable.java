package java.io;

import java.io.IOException;

//TODO public interface Closeable extends AutoCloseable {
public interface Closeable {

    public void close() throws IOException;
}
