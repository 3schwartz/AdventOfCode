import networkx as nx  # pip install networkx
import time

graph = nx.Graph()
nodes = set()
for line in open("../../data/day25_data.txt"):
    for key, values in [line.strip().split(": ")]:
        for value in values.split(" "):
            # capacity needed for nx.minimum_cut
            graph.add_edge(key, value, capacity=1.0)
            graph.add_edge(value, key, capacity=1.0)
            nodes.add(key)
            nodes.add(value)

initial = graph.copy()

start = time.time()
for f in nodes:
    found = False
    for s in nodes:
        if f == s:
            continue
        cut_count, (set_one, set_two) = nx.minimum_cut(graph, f, s)
        if cut_count == 3:
            found = True
            print(f"Part 1: {len(set_one) * len(set_two)}")
            break
    if found:
        break
print(f"Iterating through nodes: {time.time() - start}")

start = time.time()
graph = initial
minimum_edges = nx.minimum_edge_cut(graph)
if len(minimum_edges) != 3:
    assert f"length was {len(minimum_edges)}"
graph.remove_edges_from(minimum_edges)
first, second = nx.connected_components(graph)

print(f"Part 1: {len(first) * len(second)}")
print(f"Find global minimal cut: {time.time() - start}")
