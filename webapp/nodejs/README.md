
## Requirements

- A recent version of [npm][] and [nodejs][]

## Configure and run on Linux

We assume you are familiar with npm and the node.js framework.

1. In order to install dependencies run `npm intall` in the project folder.
2. In the console run the `npm start` command for starting the server.
3. Start your browser and make it point to the url `http://<host>:3000`
   
   To make it reachable by the Collabora Online server use as `<host>` the IP address of the machine where the NodeJS 
   server is running. In case the NodeJs server can't be reached you could also need to open the port 3000 on the firewall. 
   
   When both the Collabora Online server and the NodeJS server belong to the same LAN `<host>` should be a local IP address.
   
   In case the Collabora Online server is on a remote host on the internet, `<host>` must be the public IP address of 
   the machine where the NodeJS example is running.
4. In the form text field enter the address for your Collabora Online server including the protocol scheme, 
   then click on the `Load Collabora Online` button.
5. An instance of Collabora Online should appear into an iframe embedded in the same page, 
   showing a text document loaded with content a simple `Hello world!` text line.
6. Try to type in some more text ( e.g. `Hi!`) and then perform a `Save` action. 
   In the console you should get the following output:
   * `wopi PutFile endpoint`  - the PutFile wopi endpoint has been triggered
   * ` Hello World! Hi!` - the updated file content has been successfully received
    
## Note

By default the [body-parser][] node.js package used as middleware for the `PutFile` endpoint has a limit option which 
controls the maximum request body size whose default is 100kb. 
In case you want to use this package in production code, you should increase that limit accordingly to your needs.


---

[npm]: https://www.npmjs.com/get-npm
[nodejs]: https://nodejs.org/
[body-parser]: https://github.com/expressjs/body-parser
