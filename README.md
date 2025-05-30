Attempts a wasi-sockets TCP connection to the IPv4 address and port from the address variable, sends the request body or default string ping to that socket, then return any response (without buffering; whatever is in the first response packet).

```bash
# tcpbin.com; a public echo server
$ export SPIN_VARIABLE_ADDRESS=45.79.112.203:4242
$ spin up --build
[...]
# Separately: curl localhost:3000
Connecting to 45.79.112.203:4242
Sending "ping\n"
Wrote 5 bytes; waiting for response...
Got "ping\n"
```