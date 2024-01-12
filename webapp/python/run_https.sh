#!/bin/sh

if test -z "$SSL_CRT_FILE" -o -z "$SSL_KEY_FILE" ; then
    echo 'Please define SSL_CRT_FILE and SSL_KEY_FILE'
    exit 2
fi

exec python manage.py runserver_plus --cert-file $SSL_CRT_FILE --key-file $SSL_KEY_FILE
