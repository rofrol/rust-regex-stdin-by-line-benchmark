#!/bin/bash

for i in {1..64000}; do echo '54.213.178.139 - - [24/Sep/2019:03:28:55 +0200] "GET / HTTP/1.1" 301 415 "-" "Go-http-client/1.1"' >> test.log; done
