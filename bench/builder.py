from annoy import AnnoyIndex
import random
import sys

metrics = ["angular", "euclidean", "manhattan", "dot", "hamming"]
dim = int(sys.argv[1])
size = int(sys.argv[2])

for metric in metrics:
    fname = f'index.{metric}.{dim}d.ann'
    print(f'Generating index for {metric}')
    t = AnnoyIndex(dim, metric)  # Length of item vector that will be indexed
    for i in range(size):
        v = [random.gauss(0, 1) for z in range(dim)]
        t.add_item(i, v)

    t.build(20)  # 10 trees
    t.save(fname)
