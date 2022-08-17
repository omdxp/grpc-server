pbgengo:
	protoc --go_out=. --go_opt=paths=source_relative --go-grpc_out=. --go-grpc_opt=paths=source_relative pb/*.proto

server:
	cargo run

client:
	go run client/*.go

.PHONY: pbgengo server client