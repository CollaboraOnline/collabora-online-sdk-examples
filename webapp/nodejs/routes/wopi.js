var express = require('express');
var router = express.Router();

/* *
 *  wopi CheckFileInfo endpoint
 *
 *  Returns info about the file with the given document id.
 *  The response has to be in JSON format and at a minimum it needs to include
 *  the file name and the file size.
 *  The CheckFileInfo wopi endpoint is triggered by a GET request at
 *  https://HOSTNAME/wopi/files/<document_id>
 */
router.get('/files/:fileId', function(req, res) {
	console.log('file id: ' + req.params.fileId);
	// test.txt is just a fake text file
	// the Size property is the length of the string
	// returned by the wopi GetFile endpoint
	res.json({
		BaseFileName: 'test.txt',
		Size: 11,
		UserId: 1,
		UserCanWrite: true
	});
});

/* *
 *  wopi GetFile endpoint
 *
 *  Given a request access token and a document id, sends back the contents of the file.
 *  The GetFile wopi endpoint is triggered by a request with a GET verb at
 *  https://HOSTNAME/wopi/files/<document_id>/contents
 */
router.get('/files/:fileId/contents', function(req, res) {
	// we just return the content of a fake text file
	// in a real case you should use the file id
	// for retrieving the file from the storage and
	// send back the file content as response
	var fileContent = 'Hello world!';
	res.send(fileContent);
});

/* *
 *  wopi PutFile endpoint
 *
 *  Given a request access token and a document id, replaces the files with the POST request body.
 *  The PutFile wopi endpoint is triggered by a request with a POST verb at
 *  https://HOSTNAME/wopi/files/<document_id>/contents
 */
router.post('/files/:fileId/contents', function(req, res) {
	// we log to the console so that is possible
	// to check that saving has triggered this wopi endpoint
	console.log('wopi PutFile endpoint');
	if (req.body) {
		console.dir(req.body);
		console.log(req.body.toString());
		res.sendStatus(200);
	} else {
		console.log('Not possible to get the file content.');
		res.sendStatus(404);
	}
});

module.exports = router;
