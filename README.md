# yaourt

yaourt is a ssh connection manager. It lists all the hosts in your `known_hosts` ssh file and connects to one of your choice.

You need to set the `HashKnownHost` SSH option to `no` so yaourt can read the `known_hosts` file.

```sh
yaourt --help
```