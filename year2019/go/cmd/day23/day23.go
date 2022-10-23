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
	// Part1(intCodes)
	Part2(intCodes)

}

func Part2(intCodes []int) {
	manager, notify := createAddressManager()
	manager.activateNAT()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	for i := 0; i < 50; i++ {
		coder := coders.CreateNetworkCoder(i, manager.read, manager.update)
		go coder.RunNetwork(intCodes, ctx)
	}

	var throttleLast int
	for out := range notify {
		if out.address == 255 {
			fmt.Printf("NAT Received: %d, Coord: %s\n", out.address, out.coord)
			throttleLast = 0
			continue
		}
		if out.address == 0 && out.coord.IsEmpty() {
			fmt.Printf("NAT Empty Throttle: %d, Coord: %s\n", out.address, out.coord)
			throttleLast = 0
			continue
		}
		if out.address == 0 && !out.coord.IsEmpty() && throttleLast == 0 {
			fmt.Printf("NAT Throttle: %d, Coord: %s\n", out.address, out.coord)
			throttleLast += 1
			continue
		}
		if out.address == 0 && !out.coord.IsEmpty() && 0 < throttleLast && throttleLast < 50 {
			fmt.Printf("NAT Multiple: %d, Coord: %s\n", out.address, out.coord)
			throttleLast += 1
			continue
		}
		if out.address == 0 && !out.coord.IsEmpty() && throttleLast >= 50 {
			fmt.Printf("NAT High Multiple Throttle: %d, Coord: %s\n", out.address, out.coord)
			break
		}

		fmt.Printf("Address: %d, Coord: %s\n", out.address, out.coord)
		throttleLast = 0
	}
}

func Part1(intCodes []int) {
	manager, notify := createAddressManager()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	for i := 0; i < 50; i++ {
		coder := coders.CreateNetworkCoder(i, manager.read, manager.update)
		go coder.RunNetwork(intCodes, ctx)
	}

	for out := range notify {
		if out.address == 255 {
			fmt.Printf("NAT Received: %d, Coord: %s\n", out.address, out.coord)
			break
		}
		fmt.Printf("Address: %d, Coord: %s\n", out.address, out.coord)
	}
}

type addressUpdate struct {
	coord   coders.Coordinate
	address int
}

type addressManager struct {
	l             sync.RWMutex
	addresses     map[int]*collections.Queue[coders.Coordinate]
	notify        chan<- addressUpdate
	enableNAT     bool
	nat           int64
	natCoordinate coders.Coordinate
}

func createAddressManager() (*addressManager, <-chan addressUpdate) {
	notify := make(chan addressUpdate)
	manager := &addressManager{
		addresses: map[int]*collections.Queue[coders.Coordinate]{},
		notify:    notify,
	}
	return manager, notify
}

func (am *addressManager) activateNAT() {
	am.enableNAT = true
}

func (am *addressManager) update(address int, coord coders.Coordinate) {
	am.l.Lock()
	defer am.l.Unlock()
	queue, ok := am.addresses[address]
	if !ok {
		queue = collections.CreateQueue[coders.Coordinate]()
		am.addresses[address] = queue
	}
	queue.Append(coord)
	am.notify <- addressUpdate{address: address, coord: coord}

	if am.enableNAT {
		am.applyNATUpdate(address, coord)
	}
}

func (am *addressManager) applyNATUpdate(address int, coord coders.Coordinate) {
	if address == 255 {
		am.natCoordinate = coord
		return
	}
	am.nat = am.nat | (1 << address)
}

func (am *addressManager) read(address int) (bool, coders.Coordinate) {
	if am.nat == 1 && address != 0 {
		return false, coders.Coordinate{}
	}

	am.l.Lock()
	defer am.l.Unlock()
	queue, ok := am.addresses[address]
	if !ok {
		return false, coders.Coordinate{}
	}
	coord, ok := queue.TryDequeue()

	if am.enableNAT && queue.Len() == 0 {
		am.applyNATRead(address)
	}

	return ok, coord
}

func (am *addressManager) applyNATRead(address int) {
	if am.nat != 0 {
		am.nat = am.nat &^ (1 << address)
		return
	}
	queue, ok := am.addresses[0]
	if !ok {
		queue = collections.CreateQueue[coders.Coordinate]()
		am.addresses[0] = queue
	}
	if queue.Len() > 0 {
		return
	}

	queue.Append(am.natCoordinate)
	am.nat = am.nat | (1 << 0)
	am.notify <- addressUpdate{address: 0, coord: am.natCoordinate}
}
