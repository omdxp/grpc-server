package main

import (
	"context"
	"log"

	"github.com/Omar-Belghaouti/grpc-server/pb"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	cc, err := grpc.Dial("localhost:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatal(err.Error())
	}
	defer cc.Close()

	c := pb.NewEchoServiceClient(cc)

	// call echo
	req := &pb.Request{
		Msg: "Hello there",
	}
	res, err := c.Echo(context.Background(), req)
	if err != nil {
		log.Fatal(err.Error())
	}
	log.Print(res.GetMsg())
}
