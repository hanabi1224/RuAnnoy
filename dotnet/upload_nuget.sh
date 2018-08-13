dotnet --info
nuget push ${TRAVIS_BUILD_DIR}/dotnet/**/*.nupkg -Verbosity detailed -ApiKey ${NUGET_API_KEY}
