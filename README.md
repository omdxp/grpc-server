# gRPC with Rust server and Go client

Since gRPC is language agnostic, it is easy to write a server and client in multiple languages.

Here is a simple server written in Rust and a Go client.

The service is called `EchoService` and it returns the same message back to the client.

## run server

```sh
make server
```

## run client

```sh
make client
```
