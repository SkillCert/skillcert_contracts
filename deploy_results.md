
## User management deploy
**command**
```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/user_management.wasm \
  --source-account alice \
  --network testnet \
  --alias user_management
```
**result**
```
ℹ️ Simulating install transaction…
ℹ️ Signing transaction: 8e186fcd2d40f86c28d8cc93325faccc233eb445a90bd94a39f5e398dc0c553e
🌎 Submitting install transaction…
ℹ️ Using wasm hash 1e88acb5249d377a740fe1f479f646eabac1f147b15f480ce8771c7a8aaa1093
ℹ️ Simulating deploy transaction…
ℹ️ Transaction hash is 6f66ddcd3a9c26266b87febe26cf64abe6426f56e352e7a78d76d065dccb62fd
🔗 https://stellar.expert/explorer/testnet/tx/6f66ddcd3a9c26266b87febe26cf64abe6426f56e352e7a78d76d065dccb62fd
ℹ️ Signing transaction: 6f66ddcd3a9c26266b87febe26cf64abe6426f56e352e7a78d76d065dccb62fd
🌎 Submitting deploy transaction…
🔗 https://stellar.expert/explorer/testnet/contract/CBQFSNFHEK7CVTLFAKMNRZEAKL4KENV73EP7NW3CAYAAGSSCFJ3HWIGH
✅ Deployed!
CBQFSNFHEK7CVTLFAKMNRZEAKL4KENV73EP7NW3CAYAAGSSCFJ3HWIGH
```