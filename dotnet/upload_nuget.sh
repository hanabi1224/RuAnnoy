dotnet --info
nuget push ./dotnet/**/*.nupkg -Verbosity detailed -ApiKey ${NUGET_API_KEY}
