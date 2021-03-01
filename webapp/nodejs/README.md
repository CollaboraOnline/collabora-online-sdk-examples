
## Requirements

- A recent version of [npm][] and [nodejs][]

## Configure and run on Linux

We assume you are familiar with npm and the node.js framework.

1. In order to install dependencies run `npm intall` in the project folder.
2. In the console run the `npm start` command for starting the server.
3. Start your browser and make it point to the url `http://localhost:3000`
4. In the form text field enter the address for your Collabora Online server including the protocol scheme, 
   then click on the `Load Collabora Online` button.
5. An instance of Collabora Online should appear into an iframe embedded in the same page, 
   showing a text document loaded with content a simple `Hello world!` text line.
6. Try to type in some more text ( e.g. `Hi!`) and then perform a `Save` action. In the console you should get the following output:
   * `wopi PutFile endpoint`  - the PutFile wopi endpoint has been triggered
   * ` Hello World! Hi!` - the updated file content has been successfully received
    
---

[npm]: https://www.npmjs.com/get-npm
[nodejs]: https://nodejs.org/
