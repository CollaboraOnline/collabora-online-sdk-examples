Please see [../README.md](../README.md) for usage and general information.

## Requirements

- A recent version of [npm][] and [nodejs][]

## Configure and run on Linux

We assume you are familiar with npm and the node.js framework.

This sample project is made of 2 apps: the Express API server and the ReactJS client front-end.
The former is placed under the root project folder, the latter in the `client` sub-folder. 
The ReactJS front-end has its own development server, running by default on port 3000 
and forwarding api request to the Express server listening by default on port 3001.

1. In order to install dependencies for the Express app run `npm install` in the project folder.
2. In order to install dependencies for the ReactJS app run `npm install` in the `client` sub-folder.
3. In the console run the `npm start` command in the project folder for starting the Express server.
4. Open a second shell and from the `client` sub-folder run `HOST=<host> npm start` for starting the ReactJS dev server.
   Where `<host>` is the ip address or the host name of your machine.
5. Start your browser and make it point to the url `http://<host>:3000`
   
   To make it reachable by the Collabora Online server use as `<host>` the IP address of the machine where the ReactJS 
   dev server is running. In case the ReactJs server can't be reached you could also need to open the port 3000 
   and 3001 on the firewall. 
   
   When both the Collabora Online server, the ReactJS/Express server belong to the same LAN `<host>` 
   should be a local IP address.
   
   In case the Collabora Online server is on a remote host on the internet, `<host>` must be the public IP address of 
   the machine where the ReactJS example is running.
6. In the form text field enter the address for your Collabora Online server including the protocol scheme, 
   then click on the `Load Collabora Online` button.
7. An instance of Collabora Online should appear into an iframe embedded in the same page, 
   showing a text document loaded with content a simple `Hello world!` text line.
8. Try to type in some more text ( e.g. `Hi!`) and then perform a `Save` action. 
   In the console where the API server is running you should get the following output:
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
