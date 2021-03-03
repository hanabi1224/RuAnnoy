# RuAnnoy

[![main](https://github.com/hanabi1224/RuAnnoy/actions/workflows/main.yml/badge.svg)](https://github.com/hanabi1224/RuAnnoy/actions/workflows/main.yml)
[![Build status](https://img.shields.io/appveyor/ci/hanabi1224/RuAnnoy/master.svg)](https://ci.appveyor.com/project/hanabi1224/RuAnnoy)
[![Crates.io](https://img.shields.io/crates/v/ru_annoy.svg)](https://crates.io/crates/ru_annoy)
[![Coverage Status](https://coveralls.io/repos/github/hanabi1224/RuAnnoy/badge.svg?branch=master)](https://coveralls.io/github/hanabi1224/RuAnnoy?branch=master)
[![MIT License](https://img.shields.io/github/license/hanabi1224/RuAnnoy.svg)](https://github.com/hanabi1224/RuAnnoy/blob/master/LICENSE)
========
<!-- [![Build Status](https://img.shields.io/travis/hanabi1224/RuAnnoy/master.svg)](https://travis-ci.org/hanabi1224/RuAnnoy) -->

This library is a rust port of https://github.com/spotify/annoy , currently only index serving part is implemented

## Usage
```rust
use ru_annoy::*;

let index = AnnoyIndex::load(10, "index.ann", IndexType::Angular).unwrap();
let v0 = index.get_item_vector(0);
let nearest = index.get_nearest(v0.as_ref(), 5, -1, true);
```

## FFI support

### kotlin/java

#### Install via [jitpack.io](https://jitpack.io/#hanabi1224/RuAnnoy)
[![Release](https://jitpack.io/v/hanabi1224/RuAnnoy.svg)](https://jitpack.io/#hanabi1224/RuAnnoy)
```gradle
repositories {
  mavenCentral()
  maven { url 'https://jitpack.io' }
}
  
dependencies {
  implementation 'com.github.hanabi1224:RuAnnoy:0.1.2+1'
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
    <PackageReference Include="RuAnnoy" />
    <PackageReference Include="RuAnnoy-Batteries-Windows-x64" />
  </ItemGroup>
```
#### Usage
```csharp
var index = AnnoyIndex.Load("index.5d.ann", 5, IndexType.Angular);
```
