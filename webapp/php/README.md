
## Requirements

- [Apache][]

## Configuration on Linux

We assume that you have already the Apache web server installed on your machine and started, 
and that the PHP module for Apache has been installed too and loaded.

1. Edit the `apache.conf` file included in the example
   * Modify the line `Define EXAMPLE_BASE_PATH <path_to_example>` by replacing `<path_to_example>` 
     with the absolute path to the folder where you downloaded the example for instance 
     if you copied it into your home folder we have `<path_to_example> = /home/<user_name>`.
2. Edit your system-wide Apache configuration file: `/etc/apache2/default-server.conf` 
   or in case you have set up and use a virtual host the configuration file for your virtual host.
   * At the end of the config file append the following line: `IncludeOptional <path_to_example>/php/apache.conf`.
3. Restart the Apache server so that the new configuration is loaded.
   
## Run the example

1. Start your browser and make it point to the url `http://<host>/example_php`

   To make it reachable by the Collabora Online server you need to use as `<host>` the IP address of the machine where the Apache
   instance is running. In case the Apache server can't be reached you could also need to open the port for the http service on the firewall.

   When both the Collabora Online server and the Apache server belong to the same LAN `<host>` should be a local IP address.
   
   In case the Collabora Online server is on a remote host on the internet, `<host>` must be the public IP address of
   the machine where the Apache server is running.
   
   In case you enabled SSL support for Apache, you should use `https` in place of `http`.
2. In the form text field enter the address for your Collabora Online server including the protocol scheme, 
   then click on the `Load Collabora Online` button.
3. An instance of Collabora Online should appear into an iframe embedded in the same page, 
   showing a text document loaded with content a simple `Hello world!` text line.
4. Try to type in some more text ( e.g. `Hi!`) and then perform a `Save` action. Now check Apache error log file, 
   you should see 2 log lines as the following ones:
   * `wopiPutFile invoked` - the PutFile wopi endpoint has been triggered
   * `\xef\xbb\xbfHello World! Hi!\n` - the updated file content has been successfully received
    

---

[apache]: https://httpd.apache.org/docs