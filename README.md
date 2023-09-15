# wikijs-rs
API bindings, CLI client and FUSE filesystem for Wiki.js written in Rust.

## State of Project
This project is still very early in its life, so read through the following
lines to know what's usable already.

**TL;DR:** library: yes, cli: not everything is there yet but yes, fuse: no

### Library
All GraphQL queries and mutations as well as asset upload and download via the
REST API have their counterpart functions in the library. Those functions
should work as long as the APIs behave as expected from their specification.
Not everything has been tested yet, but it should have decent error handling
and thus be in a fairly usable state by now.

### CLI
The CLI client at the moment only implements a small part of the functions the
library offers mostly concentrating on the page module. If offers things like
listing pages, showing their metadata, editing their content via your editor,
and deleting them for example. Since it mostly just interfaces with the library
and also handles its errors it should be usable, too.

### FUSE
This is still heavily work-in-progress. So be very careful when trying it out!

## Contributing
Two big part where contributing is fairly easy and needed are writing tests
and documentation.
