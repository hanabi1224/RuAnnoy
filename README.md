# RuAnnoy

[![main](https://github.com/hanabi1224/RuAnnoy/actions/workflows/main.yml/badge.svg)](https://github.com/hanabi1224/RuAnnoy/actions/workflows/main.yml)
[![MIT License](https://img.shields.io/github/license/hanabi1224/RuAnnoy.svg)](https://github.com/hanabi1224/RuAnnoy/blob/master/LICENSE)
========

This library is a rust port of [spotify/annoy](https://github.com/spotify/annoy) , currently only index serving is supported.

A live demo using web assembly is available at https://annoy-web-demo.vercel.app/

It also provides [FFI bindings](https://github.com/hanabi1224/RuAnnoy#ffi-support) for [jvm](https://github.com/hanabi1224/RuAnnoy#kotlinjava), [dotnet](https://github.com/hanabi1224/RuAnnoy#dotnet) and [dart](https://github.com/hanabi1224/RuAnnoy#dart)

| Metric    | Serve | Build | jvm binding | dotnet binding | dart binding | WASM support |
| :-------- | :---: | ----: | ----------- | -------------- | ------------ | ------------ |
| Angular   |  ✅   |    ❌ | ✅          | ✅             | ✅           | ✅           |
| Euclidean |  ✅   |    ❌ | ✅          | ✅             | ✅           | ✅           |
| Manhattan |  ✅   |    ❌ | ✅          | ✅             | ✅           | ✅           |
| Dot       |  ✅   |    ❌ | ✅          | ✅             | ✅           | ✅           |
| Hamming   |  ❌   |    ❌ | ❌          | ❌             | ❌           | ❌           |

### Install via [crates.io](https://crates.io/crates/annoy-rs)

[![Crates.io](https://img.shields.io/crates/v/annoy-rs.svg)](https://crates.io/crates/annoy-rs)
[![codecov](https://codecov.io/gh/hanabi1224/RuAnnoy/branch/master/graph/badge.svg?token=jVO7N0AVTH)](https://codecov.io/gh/hanabi1224/RuAnnoy)
[![dependency status](https://deps.rs/repo/github/hanabi1224/RuAnnoy/status.svg?style=flat-square)](https://deps.rs/repo/github/hanabi1224/RuAnnoy)

```toml
# Cargo.toml
[dependencies]
annoy-rs = "0.1"
```

### Usage

```rust
use annoy_rs::*;

let index = AnnoyIndex::load(10, "index.ann", IndexType::Angular).unwrap();
let v0 = index.get_item_vector(0);
let nearest = index.get_nearest(v0.as_ref(), 5, -1, true);
```

## SIMD support

SIMD is supported via [`std::simd`](https://doc.rust-lang.org/nightly/std/simd/index.html) on nightly rust. Note that avx intrinsics need to be enabled explicitly by setting your cpu features in `RUSTFLAGS` environment variable.

```
RUSTFLAGS="-Ctarget-feature=+avx" cargo +nightly build --release
# or
RUSTFLAGS="-Ctarget-cpu=native" cargo +nightly build --release
```

## WASM support

Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```
wasm-pack build
wasm-pack test --node
```

simd128 is supported in chrome by default.

To enable simd128, build with below command

```
RUSTFLAGS="-Ctarget-feature=+simd128" cargo +nightly build --release --target wasm32-unknown-unknown
```

An example site is deployed at https://annoy-web-demo.vercel.app/

Source code is under [example/web](https://github.com/hanabi1224/RuAnnoy/tree/master/example/web)

## FFI support

### kotlin/java

It uses JNI bindings to rust crate and is ~5-10x faster than [pure java implementation](https://github.com/spotify/annoy-java) in [benchmark scenario](https://github.com/hanabi1224/RuAnnoy/tree/master/bench)

Note that the prebuilt dynamically linked libraries are built with simd support, avx cpu feature is required.

#### Install via [jitpack.io](https://jitpack.io/#hanabi1224/RuAnnoy)

[![Release](https://jitpack.io/v/hanabi1224/RuAnnoy.svg)](https://jitpack.io/#hanabi1224/RuAnnoy)

```gradle
repositories {
  mavenCentral()
  maven { url 'https://jitpack.io' }
}

dependencies {
  implementation 'com.github.hanabi1224:RuAnnoy:<tag>'
}
```

#### Usage

```kotlin
val index = AnnoyIndex.tryLoad("index.5d.ann", 5, IndexType.Angular)
```

### dotnet

| Runtimes                      | Nuget package                                                                                                                                 |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| RuAnnoy                       | [![NuGet version](https://buildstats.info/nuget/RuAnnoy)](https://www.nuget.org/packages/RuAnnoy)                                             |
| RuAnnoy-Batteries-Windows-x64 | [![NuGet version](https://buildstats.info/nuget/RuAnnoy-Batteries-Windows-x64)](https://www.nuget.org/packages/RuAnnoy-Batteries-Windows-x64) |
| RuAnnoy-Batteries-Linux-x64   | [![NuGet version](https://buildstats.info/nuget/RuAnnoy-Batteries-Linux-x64)](https://www.nuget.org/packages/RuAnnoy-Batteries-Linux-x64)     |
| RuAnnoy-Batteries-Darwin-x64  | [![NuGet version](https://buildstats.info/nuget/RuAnnoy-Batteries-Darwin-x64)](https://www.nuget.org/packages/RuAnnoy-Batteries-Darwin-x64)   |

#### Install via nuget

```xml
  <ItemGroup>
    <PackageReference Include="RuAnnoy" Version="*" />
    <PackageReference Include="RuAnnoy-Batteries-Windows-x64" Version="*" />
  </ItemGroup>
```

#### Usage

```csharp
var index = AnnoyIndex.Load("index.5d.ann", 5, IndexType.Angular);
```

### dart

#### Install via [pub.dev](https://pub.dev/packages/dart_native_annoy)

```yaml
# pubspec.yaml
dependencies:
  dart_native_annoy: ^0.1.0
```

#### Usage

```dart
import 'dart:ffi';
import 'package:dart_native_annoy/annoy.dart';

/// Creat factory from DynamicLibrary
final indexFactory = AnnoyIndexFactory(lib: DynamicLibrary.open('libannoy_rs_ffi.so'));

/// Load index
final index = indexFactory.loadIndex(
      'index.euclidean.5d.ann', 5, IndexType.Euclidean)!;

print('size: ${index.size}');

final v3 = index.getItemVector(3);

final nearest = index.getNearest(v0, 5, includeDistance: true);
```

## TODO

- Index building support
- CLI tool to build index from file
