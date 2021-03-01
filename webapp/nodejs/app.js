var express = require('express');
var path = require('path');
var logger = require('morgan');
const bodyParser = require('body-parser');

var indexRouter = require('./routes/index');
var wopiRouter = require('./routes/wopi');

var app = express();

app.use(logger('dev'));
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(bodyParser.raw());
app.use(express.static(path.join(__dirname, 'public')));

app.use('/', indexRouter);
app.use('/wopi', wopiRouter);

module.exports = app;
