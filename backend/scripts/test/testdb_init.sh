#!/bin/bash
# Description:
#     A simple script to initialize database for tests. This script need to be executed before running tests.
#     If you run tests through nextest (e.g., cargo nextest run), this script will be executed automatically.
# Usage:
#     ./init.sh <dbdump>

psql -U postgres -c "DROP DATABASE IF EXISTS \"booklyn-test\";" > /dev/null
psql -U postgres -c "CREATE DATABASE \"booklyn-test\";" > /dev/null
psql -U postgres -d booklyn-test < $1 > /dev/null
