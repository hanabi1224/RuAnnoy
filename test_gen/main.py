from annoy import AnnoyIndex
import random

metrics = ["angular", "euclidean", "manhattan","dot","hamming"]
dim = 5
size = 100

for metric in metrics:
    fname = f'index.{metric}.{dim}d.ann'
    print(f'Generating index for {metric}')
    # t = AnnoyIndex(dim, metric)  # Length of item vector that will be indexed
    # for i in range(size):
    #     v = [random.gauss(0, 1) for z in range(dim)]
    #     t.add_item(i, v)

    # t.build(10)  # 10 trees
    # t.save(fname)

    # ...

    u = AnnoyIndex(dim, metric)
    u.verbose(True)
    u.load('./../tests/'+fname)  # super fast, will just mmap the file
    print(u.get_item_vector(3))
    v0 = u.get_item_vector(0)
    print(v0)
    nearests = u.get_nns_by_vector(v0, 5, include_distances=True)
    id_1 = nearests[0][1]    
    print(u.get_item_vector(id_1))
    print(u.get_distance(0, id_1))
    # print(u.get_distance(0, 16))
    print(nearests[0])  # will find the 1000 nearest neighbors
    print(nearests[1])
