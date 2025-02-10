# Choice Exchange Smart Contracts

Choice Exchange is an AMM protocol forked from TerraSwap

## Build

For a production-ready (compressed) build, run the following from the repository root:

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.16.1
```

The optimized contracts are generated in the artifacts/ directory.
