import express from 'express';
import path from 'path';

let router = express.Router();
export default router;

export const TEST_TOKEN = 'test';
export const TEST_SETTINGS_TOKEN = 'testsettings';

/* *
 *  wopi CheckFileInfo endpoint
 *
 *  Returns info about the file with the given document id.
 *  The response has to be in JSON format and at a minimum it needs to include
 *  the file name and the file size.
 *  The CheckFileInfo wopi endpoint is triggered by a GET request at
 *  https://HOSTNAME/wopi/files/<document_id>
 */
router.get('/files/:fileId', (req, res) => {
	console.log('file id: ' + req.params.fileId);
	let stamp = Date.now();
	let baseUrl = `${req.protocol}://${req.host}${req.baseUrl}`;
	let accessToken = req.query['access_token'];
	// test.txt is just a fake text file
	// the Size property is the length of the string
	// returned by the wopi GetFile endpoint
	res.json({
		BaseFileName: 'test.txt',
		Size: 11,
		UserId: 1,
		UserCanWrite: true,
		EnableInsertRemoteImage: true,
		UserSettings: {
			uri: `${baseUrl}/settings?type=userconfig&access_token=${accessToken}&fileId=-1`,
			stamp,
		},
		SharedSettings: {
			uri: `${baseUrl}/settings?type=systemconfig&access_token=${accessToken}&fileId=-1`,
			stamp,
		}
	});
});

/* *
 *  wopi GetFile endpoint
 *
 *  Given a request access token and a document id, sends back the contents of the file.
 *  The GetFile wopi endpoint is triggered by a request with a GET verb at
 *  https://HOSTNAME/wopi/files/<document_id>/contents
 */
router.get('/files/:fileId/contents', (req, res) => {
	// we just return the content of a fake text file
	// in a real case you should use the file id
	// for retrieving the file from the storage and
	// send back the file content as response
	const fileContent = 'Hello world!';
	res.send(fileContent);
});

/* *
 *  wopi PutFile endpoint
 *
 *  Given a request access token and a document id, replaces the files with the POST request body.
 *  The PutFile wopi endpoint is triggered by a request with a POST verb at
 *  https://HOSTNAME/wopi/files/<document_id>/contents
 */
router.post('/files/:fileId/contents', (req, res) => {
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

let settings = {};

router.get('/settings', (req, res) => {
	let t = req.query['type'];
	let token = req.query['access_token'];
	let {fileId} = req.query;
	if (token != TEST_SETTINGS_TOKEN && token != TEST_TOKEN) {
		res.sendStatus(401);
		return;
	}
	console.log(`setting.get type=${t}`);
	if (fileId == -1) {
		let kind;
		switch (t) {
		case 'userconfig':
			kind = 'user';
			break;
		case 'systemconfig':
			kind = 'shared';
			break;
		default:
			res.sendStatus(400);
			return;
		}

		let s = Object.create(settings);
		s.kind = kind;
		res.json(s);
	} else {
		res.sendStatus(400);
	}
});

router.delete('/settings', (req, res) => {
	console.log('setting.delete');
	let { fileId } = req.query
	delete settings[fileId];
	res.sendStatus(200);
});

router.post('/settings/upload', (req, res) => {
	console.log(`setting.post`);
	let {fileId} = req.query;
	if (!fileId) {
		res.sendStatus(400);
		return;
	}
	let filename = path.basename(fileId);
	let stamp = Date.now();
	let baseUrl = req.baseUrl;
	console.log(`setting.post returning ${stamp} ${filename} ${baseUrl}${fileId}`);
	settings[fileId] = req.body;
	res.json({
		status: 'success',
		filename,
		details: {
			stamp,
			uri: baseUrl + fileId,
		}
	});
});
