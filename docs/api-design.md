

# API Design

This document aims to be a reference and scratchpad for API design notes.


## cli args standards

### Notes

- for optional args that are also optional in the config file, setting it to
  NONE on the cli will override any set in the config file so will be the
  default as if none set.

### List of args

These should be consistently named between backends and have the same semantics.

* -u --url
* -d --domain
* -p --port
* -s --syntax (NONE-able, passed directly to api so some backends might be
  different (eg. py instead of python))
* -t --title (NONE-able)
* -e --expires (NONE-able, time to live in a humantime duration or a string for special server supported values)
* -P --password (NONE-able, password to view a paste)
* -r --recipient (NONE-able, for servers that can email link to a recipient
  address)
* -k --apikey (NONE-able, used for auth password or apikey/token)
* -U --username (NONE-able, used for auth, semantically different to author)
* -a --author (NONE-able, poster/uploader name)
* -R --reads (NONE-able, n reads before paste is deleted)
