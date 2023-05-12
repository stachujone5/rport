<h1 align="center">
  rport
</h1>

<h4 align="center">Simple CLI application to list ports that are currently in use and kill processes running on them</h4>

## Usage

Show all currently used TCP ports:

```sh
rport -l
```

Kill a process that is running on a specified port:

```sh
rport -k 3000
```

Kill multiple processes:

```sh
rport -k 3000 3001 3002
```

## Flags

- -h, --help, Display all available commands
- -l, --list, Display all ports that are currently in use
- -k, --kill, Kill a port by providing a port number

> :warning: **Works only on linux**

## License

MIT
