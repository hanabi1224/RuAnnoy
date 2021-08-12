import 'package:dart_native_annoy/annoy.dart';

import 'setup_util.dart';

extension EnumParser on String {
  T toEnum<T>(List<T> values) {
    return values.firstWhere(
        (e) =>
            e.toString().toLowerCase().split(".").last == '$this'.toLowerCase(),
        orElse: () => values[0]);
  }
}

void main(List<String> arguments) async {
  final int dim = int.parse(arguments[0]);
  final int size = int.parse(arguments[1]);
  final int nResult = int.parse(arguments[2]);
  final int nLoop = int.parse(arguments[3]);

  final lib = await SetupUtil.getDylibAsync();
  final fac = AnnoyIndexFactory(lib);

  for (final metric in ['angular', 'euclidean', 'manhattan', 'dot']) {
    final path = '../index.${metric}.${dim}d.ann';
    final index =
        fac.loadIndex(path, dim, metric.toEnum<IndexType>(IndexType.values))!;
    // print(index.size);
    final sw = Stopwatch();
    sw.start();
    for (var i = 0; i < nLoop; i++) {
      final id = i % size;
      final v = index.getItemVector(id);
      final _ = index.getNearest(v, nResult, includeDistance: true);
    }
    sw.stop();
    print("[Dart]dart_native_annoy");
    print("${metric} Total time elapsed: ${sw.elapsedMilliseconds / 1000}s");
    print("${metric} Avg time elapsed: ${sw.elapsedMilliseconds / nLoop}ms");
    print('');
    index.close();
  }
}
