'use strict';

let express = require('express');
let path = require('path');
let logger = require('morgan');
let bodyParser = require('body-parser');

let indexRouter = require('./routes/index');
let wopiRouter = require('./routes/wopi');

// maximum request body size handled by the bodyParser package
// increase it if you need to handle larger files
const maxDocumentSize = '100kb';

let app = express();

app.use(logger('dev'));
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(bodyParser.raw({limit: maxDocumentSize}));
app.use(express.static(path.join(__dirname, 'public')));

app.use('/', indexRouter);
app.use('/wopi', wopiRouter);

module.exports = app;
