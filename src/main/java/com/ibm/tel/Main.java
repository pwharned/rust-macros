package com.ibm.tel;
import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.io.File;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.io.OutputStream;
import java.io.InputStreamReader;
import java.io.BufferedReader;
import java.util.Objects;

public class Main {

  public static void main(String[] args) throws Exception {
      if (Objects.equals(System.getenv().getOrDefault("DEV", "0"), "1")){
          Thread thread1 = new Thread(() -> {
              try {
                  DirectoryWatcher.main(args);

              } catch (Exception e) {
                  System.out.println(e.getMessage());
              }
          });
          Thread thread2 = new Thread(() -> {
              try {
                  runBinary();
              } catch (Exception e) {
                  throw new RuntimeException(e);
              }
          });
          thread1.start();
          thread2.start();
      }else {
          DirectoryWatcher.createStatic();

          runBinary();
      }

  }

    public static void runBinary() throws Exception {
        try {
            // Extract the binary file from the JAR
            File binaryFile = extractBinaryFile("server");

            // Make the binary file executable (if necessary)
            binaryFile.setExecutable(true);

            // Execute the binary file
            ProcessBuilder processBuilder = new ProcessBuilder(binaryFile.getAbsolutePath());
            Process process = processBuilder.start();
            BufferedReader errorReader = new BufferedReader(new InputStreamReader(process.getErrorStream()));
            StringBuilder errorMessage = new StringBuilder();
            String line;
            while ((line = errorReader.readLine()) != null) {
                System.out.println(line);

            }
            // Wait for the process to complete
            int exitCode = process.waitFor();
            System.out.println("Binary file executed with exit code: " + exitCode);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }











    private static File extractBinaryFile(String resourceName) throws IOException {
        // Get the input stream for the binary file from the JAR
        InputStream inputStream = Main.class.getClassLoader().getResourceAsStream(resourceName);
        if (inputStream == null) {
            throw new IOException("Resource not found: " + resourceName);
        }

        // Create a temporary file to store the binary
        File tempFile = File.createTempFile("tempBinary", ".bin");
        tempFile.deleteOnExit(); // Ensure the file is deleted when the JVM exits

        // Copy the binary file to the temporary file
        try (OutputStream outputStream = new FileOutputStream(tempFile)) {
            byte[] buffer = new byte[1024];
            int bytesRead;
            while ((bytesRead = inputStream.read(buffer)) != -1) {
                outputStream.write(buffer, 0, bytesRead);
            }
        }

        return tempFile;
    }



    public static void run(String[] args) {
        List<String> command = Arrays.asList("./resources/bin/server"); // Example command
        ProcessBuilder processBuilder = new ProcessBuilder(command);
        processBuilder.redirectErrorStream(true); // Combine error and output streams

        try {
            Process process = processBuilder.start();
            // Read the output of the process
            java.util.Scanner s = new java.util.Scanner(process.getInputStream()).useDelimiter("\\A");
            String output = s.hasNext() ? s.next() : "";
            System.out.println(output);

            int exitCode = process.waitFor();
            System.out.println("\nExited with error code : " + exitCode);

        } catch (IOException | InterruptedException e) {
            e.printStackTrace();
        }
    }

}
