import express from 'express';
import path from 'path';
import logger from 'morgan';
import bodyParser from 'body-parser';

import indexRouter from './routes/index.js';
import wopiRouter from './routes/wopi.js';

// maximum request body size handled by the bodyParser package
// increase it if you need to handle larger files
const maxDocumentSize = '100kb';

let app = express();
export default app;

app.use(logger('dev'));
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(bodyParser.raw({limit: maxDocumentSize}));
let static_dir = new URL('../html', import.meta.url);
app.use(express.static(static_dir.pathname));

app.use('/', indexRouter);
app.use('/wopi', wopiRouter);
