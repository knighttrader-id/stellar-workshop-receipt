Deploy To Testnet

1. Membuat Wallet
    stellar keys generate stellar-wallet --network testnet --fund
2. Build Smart Contract
    stellar contract build
3. Deploy
    stellar contract deploy --source-account stellarwallet

✅ Build Complete

ℹ️  Uploading contract WASM…
ℹ️  Simulating transaction…
ℹ️  Signing transaction: ee8b8bfe2897751ccabb37f51e56333a2a99057cae4e845d1f90e41875ad0ed2
🌎 Sending transaction…
✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/ee8b8bfe2897751ccabb37f51e56333a2a99057cae4e845d1f90e41875ad0ed2
ℹ️  Deploying contract using wasm hash 93eb9beb66ddc79fe51eed3b2de3817f23efd0e5fb748b8f3209975fef39a43c
ℹ️  Simulating transaction…
ℹ️  Signing transaction: cb649c930ebc8ff2e52af198eb2648b1242e8d76faa84ae6f4e6f005e18aebee
🌎 Sending transaction…
✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/cb649c930ebc8ff2e52af198eb2648b1242e8d76faa84ae6f4e6f005e18aebee
🔗 https://lab.stellar.org/r/testnet/contract/CCKG4Z6ZYYT4QTPV4JQ3MLQVK4CU73ZL5FG3QQI27J3FKHIX2TNHVX2Z
✅ Deployed!
CCKG4Z6ZYYT4QTPV4JQ3MLQVK4CU73ZL5FG3QQI27J3FKHIX2TNHVX2Z