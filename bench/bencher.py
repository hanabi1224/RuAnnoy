from annoy import AnnoyIndex

import sys
import time

if __name__ == '__main__':
    dim = int(sys.argv[1])
    size = int(sys.argv[2])
    n_result = int(sys.argv[3])
    n_loop = int(sys.argv[4])
    for metric in [
        'angular',
        'euclidean',
        'manhattan',
        'dot'
    ]:
        fp = f'index.{metric}.{dim}d.ann'
        index = AnnoyIndex(dim, metric)
        index.load(fp)
        t_start = time.perf_counter()
        for i in range(n_loop):
            id = i % size
            v = index.get_item_vector(id)
            r = index.get_nns_by_vector(v, n_result, -1, True)
        t_end = time.perf_counter()
        diff = t_end - t_start
        print("[Python]annoy")
        print(f"{metric} Total time elapsed: {diff:.3g}s")
        print(f"{metric} Avg time elapsed: {diff*1000/n_loop:.3g}ms")
        print()
