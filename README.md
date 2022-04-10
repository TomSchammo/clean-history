# Clean History

A daemon to automatically clean shell histories.

Having my shell history polluted by duplicate commands
or commands that are specific to a certain folder or file
that no longer exists is annoying.

So I decided to do something about it.

## Features

### Z-Shell

The program can automatically find the history file of the Z-Shell (generally).

### Removal of duplicates

The program filters out duplicates from your history.


## Command line options

- `--daemonize` or `-d` runs the program as a background process in a loop.
- `--timeout <TIMEOUT>` or `-t <TIMEOUT>` overwrites the default timeout of 1s between iterations when running as a daemon.
- `--history <HISTFILE_PATH>` or `-h <HISTFILE_PATH>` allows the user to provide a path to their history file.
  This is generally more robust and therefore recommended.

## Roadmap

- [ ] Intelligent file tracking to remove any commands that access/
      interact with files that no longer exist
- [ ] Support for automatically finding bash_history (maybe)
