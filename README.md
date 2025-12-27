# yaourt

yaourt is a ssh connection manager. It lists all the hosts in your `known_hosts` ssh file and connects to one of your choice.

You need to set the `HashKnownHost` SSH option to `no` so yaourt can read the `known_hosts` file.

## Build

You need rust installed to build the project

```sh
cargo build --release
```

## Install

You can then install it directly, for example:

```sh
cp target/release/yaourt usr/bin/yaourt
```

## Usage

```sh
yaourt <SEARCH>
```

Where `SEARCH` would typically be the name (or more realistically, part of the name) of a host you'd like to connect to. 

You can also display the help message to figure it out by yourself and discover more options:

```sh
yaourt --help
```