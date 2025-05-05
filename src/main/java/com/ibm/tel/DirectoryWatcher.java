package com.ibm.tel;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.net.URL;
import java.nio.file.*;
import java.util.ArrayList;
import java.util.Enumeration;
import java.util.List;
import java.util.jar.JarEntry;
import java.util.jar.JarFile;

public class DirectoryWatcher {

    public static List<String> getResources() {
        List<String> entryNames = new ArrayList<>();
        try {
            // Get the URL of the JAR file
            URL jarUrl = Main.class.getProtectionDomain().getCodeSource().getLocation();

            // Convert the URL to a file path
            String jarPath = jarUrl.toURI().getPath();

            // Open the JAR file
            JarFile jarFile = new JarFile(jarPath);

            // Get an enumeration of the entries in the JAR file
            Enumeration<JarEntry> entries = jarFile.entries();

            // Iterate through the entries and print their names
            while (entries.hasMoreElements()) {
                JarEntry entry = entries.nextElement();
                entryNames.add(entry.getName());
            }

            // Close the JAR file
            jarFile.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
        return entryNames;
    }


    public static List<String> getFilesFromDirectory(String directoryPath) {
        List<String> fileNames = new ArrayList<>();
        try {
            Path directory = Paths.get(directoryPath);
            DirectoryStream<Path> stream = Files.newDirectoryStream(directory);
            for (Path path : stream) {
                if (Files.isRegularFile(path)) {
                    fileNames.add(directoryPath + "/" + path.getFileName().toString());
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return fileNames;
    }


    public static void createStatic() throws Exception {
        // Get the ClassLoader
        ClassLoader classLoader = DirectoryWatcher.class.getClassLoader();
        List<String> resources = getResources();



        // Create the output directory
        File outputDir = new File("static");
        outputDir.mkdirs();

        // Iterate over the resources and write them to the output directory
        for (int i = 0; i < resources.size(); i++){
            String resource = resources.get(i);
            String resourceName = resource.substring(resource.lastIndexOf('/') + 1);
            File outputFile = new File(outputDir, resourceName);
            if (!resource.startsWith("static")){
                continue;
            }
            if (outputFile.isDirectory() ){
                File dir = new File(outputDir, resourceName.substring(resourceName.lastIndexOf('/') + 1));
                dir.mkdirs();

            }else{
                System.out.println(resourceName);

                URL res = classLoader.getResource(resource);

                try (InputStream inputStream = res.openStream();
                     FileOutputStream outputStream = new FileOutputStream(outputFile)) {
                    byte[] buffer = new byte[1024];
                    int bytesRead;
                    while ((bytesRead = inputStream.read(buffer)) != -1) {
                        outputStream.write(buffer, 0, bytesRead);
                    }
                }
            }
        }
    }


    public static void createStaticDev() throws Exception {
        // Get the ClassLoader
        ClassLoader classLoader = DirectoryWatcher.class.getClassLoader();
        List<String> resources = getFilesFromDirectory("src/main/resources/static");



        // Create the output directory
        File outputDir = new File("static");
        outputDir.mkdirs();

        // Iterate over the resources and write them to the output directory
        for (int i = 0; i < resources.size(); i++){
            String resource = resources.get(i);
            if (resource.endsWith("~")){
                continue;
            }

            String resourceName = resource.substring(resource.lastIndexOf('/') + 1);
            System.out.println(resource);
            File outputFile = new File(outputDir, resourceName);

            if (outputFile.isDirectory() ){
                File dir = new File(outputDir, resourceName.substring(resourceName.lastIndexOf('/') + 1));
                dir.mkdirs();

            }else{
                System.out.println(resourceName);


                Files.copy(Paths.get(resource), Paths.get(outputFile.toURI()), StandardCopyOption.REPLACE_EXISTING);

            }
        }
    }


    public static void main(String[] args) {
        try {
            // Define the directory to watch
            Path directory = Paths.get("src/main/resources/static");

            // Create a WatchService
            WatchService watchService = FileSystems.getDefault().newWatchService();

            // Register the directory with the WatchService, specifying the events to watch for
            directory.register(watchService, StandardWatchEventKinds.ENTRY_CREATE,
                    StandardWatchEventKinds.ENTRY_MODIFY, StandardWatchEventKinds.ENTRY_DELETE);

            System.out.println("Watching directory: " + directory);

            // Start the watch loop
            while (true) {
                WatchKey key;
                try {
                    // Wait for a watch key to be available
                    key = watchService.take();
                } catch (InterruptedException e) {
                    return;
                }

                // Process the events for the key
                for (WatchEvent<?> event : key.pollEvents()) {
                    WatchEvent.Kind<?> kind = event.kind();

                    // Handle different event types
                    if (kind == StandardWatchEventKinds.OVERFLOW) {
                        System.err.println("Event overflow occurred.");
                        continue;
                    }

                    // Get the file name associated with the event
                    WatchEvent<Path> pathEvent = (WatchEvent<Path>) event;
                    Path fileName = pathEvent.context();
                    if (fileName.endsWith("~")){
                        continue;
                    }

                    // Print the event information
                    System.out.println("Event: " + kind.name() + ", File: " + fileName);
                    createStaticDev();
                }

                // Reset the key to receive further events
                boolean valid = key.reset();
                if (!valid) {
                    break; // Exit the loop if the key is no longer valid
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}