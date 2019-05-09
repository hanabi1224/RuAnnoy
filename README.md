# RuAnnoy

[![Build Status](https://img.shields.io/travis/hanabi1224/RuAnnoy/master.svg)](https://travis-ci.org/hanabi1224/RuAnnoy)
[![Build status](https://img.shields.io/appveyor/ci/hanabi1224/RuAnnoy/master.svg)](https://ci.appveyor.com/project/hanabi1224/RuAnnoy)
[![Crates.io](https://img.shields.io/crates/v/ru_annoy.svg)](https://crates.io/crates/ru_annoy)
[![Coverage Status](https://coveralls.io/repos/github/hanabi1224/RuAnnoy/badge.svg?branch=master)](https://coveralls.io/github/hanabi1224/RuAnnoy?branch=master)
[![MIT License](https://img.shields.io/github/license/hanabi1224/RuAnnoy.svg)](https://github.com/hanabi1224/RuAnnoy/blob/master/LICENSE)
========

This library is a rust port of https://github.com/spotify/annoy , currently only index serving part is implemented

## FFI support
### dotnet nuget packages

| Runtimes                      | Nuget package                                                                                                                                 |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| RuAnnoy                       | [![NuGet version](https://buildstats.info/nuget/RuAnnoy)](https://www.nuget.org/packages/RuAnnoy)                                             |
| RuAnnoy-Batteries-Windows-x64 | [![NuGet version](https://buildstats.info/nuget/RuAnnoy-Batteries-Windows-x64)](https://www.nuget.org/packages/RuAnnoy-Batteries-Windows-x64) |
| RuAnnoy-Batteries-Linux-x64   | [![NuGet version](https://buildstats.info/nuget/RuAnnoy-Batteries-Linux-x64)](https://www.nuget.org/packages/RuAnnoy-Batteries-Linux-x64)     |
| RuAnnoy-Batteries-Darwin-x64  | TODO                                                                                                                                          |

#### Installation
```xml
  <ItemGroup>
    <PackageReference Include="RuAnnoy" />
    <PackageReference Include="RuAnnoy-Batteries-Windows-x64" />
  </ItemGroup>
```
