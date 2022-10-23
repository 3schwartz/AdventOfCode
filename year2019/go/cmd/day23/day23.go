package main

import (
	"advent/pkg/coders"
	"advent/pkg/collections"
	"advent/pkg/read"
	"context"
	"fmt"
	"sync"
)

func main() {
	codes := read.ReadData("day23")
	intCodes := coders.ParseIntCodes(codes)
	notify := make(chan addressUpdate)
	manager := createAddressManager(notify)

	ctx, cancel := context.WithCancel(context.Background())
	for i := 0; i < 50; i++ {
		coder := coders.CreateNetworkCoder(i, manager.Read, manager.Update)
		go coder.RunNetwork(intCodes, ctx)
	}

	for out := range notify {
		fmt.Printf("Address: %d, Coord: %s\n", out.address, out.coord)
		if out.address == 255 {
			close(notify)
		}
	}

	cancel()
}

type addressUpdate struct {
	coord   coders.Coordinate
	address int
}

type addressManager struct {
	l         sync.RWMutex
	addresses map[int]*collections.Queue[coders.Coordinate]
	notify    chan<- addressUpdate
}

func createAddressManager(notify chan<- addressUpdate) *addressManager {
	return &addressManager{
		addresses: map[int]*collections.Queue[coders.Coordinate]{},
		notify:    notify,
	}
}

func (am *addressManager) Update(address int, coord coders.Coordinate) {
	am.l.Lock()
	defer am.l.Unlock()
	queue, ok := am.addresses[address]
	if !ok {
		queue = collections.CreateQueue[coders.Coordinate]()
		am.addresses[address] = queue
	}
	queue.Append(coord)
	am.notify <- addressUpdate{address: address, coord: coord}
}

func (am *addressManager) Read(address int) (bool, coders.Coordinate) {
	am.l.RLock()
	defer am.l.RUnlock()
	queue, ok := am.addresses[address]
	if !ok {
		return false, coders.Coordinate{}
	}
	coord, ok := queue.TryDequeue()

	return ok, coord
}
