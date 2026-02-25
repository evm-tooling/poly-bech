# EVM-RPC Benchmarks

Standalone poly-bench suites derived from `debug/bench/` Go and TypeScript benchmarks. Each file mirrors the corresponding tests for fair viem-go vs viem comparison.

## Pure CPU (no Anvil)

| File | Suite | Benchmarks |
|------|-------|------------|
| `unit.bench` | Unit parsing/formatting | parseEther, formatEther, parseUnits, parseGwei, formatUnits |
| `abi.bench` | ABI encode/decode | encodeSimple, encodeComplex, encodeMultiArg, decodeResult, encodePacked |
| `address.bench` | Address utilities | isAddress, checksum, create, create2 |
| `hash.bench` | Hashing | keccak256, sha256, functionSelector, eventSelector |
| `ens.bench` | ENS | namehash, labelhash, normalize |
| `event.bench` | Event decoding | decodeTransfer, decodeBatch10, decodeBatch100 |
| `signature.bench` | Signatures | hashMessage, recoverAddress, verifyMessage, parseSignature |

## RPC (requires Anvil)

| File | Suite | Benchmarks |
|------|-------|------------|
| `call.bench` | Contract calls | callBasic, callWithData, callWithAccount, callDecimals, callSymbol |
| `multicall.bench` | Multicall | multicallBasic, multicallWithArgs, multicallMultiContract, multicall10, multicallDeployless, multicallTokenMetadata |

## Run

```bash
# Pure CPU (no network)
poly-bench run benchmarks/evm-rpc/unit.bench
poly-bench run benchmarks/evm-rpc/abi.bench
# ... etc

# RPC (spawns Anvil, forks mainnet)
poly-bench run benchmarks/evm-rpc/call.bench
poly-bench run benchmarks/evm-rpc/multicall.bench
```

## Dependencies

Uses `viem-go` (Go) and `viem` (TypeScript) from `polybench.toml`. No extra deps needed.
