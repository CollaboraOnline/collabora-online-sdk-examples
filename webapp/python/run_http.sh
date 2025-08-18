#!/bin/sh


# TODO
# important!  IPv6+IPv4 needed  // BAD, listens publicly
# IPv4 won't work, COOL expects IPv6 host!
# IPv6 won't work, Firefox doesn't allow embedding v4 in v6 localhost
exec python3 manage.py runserver_plus '[::0]:3000'
