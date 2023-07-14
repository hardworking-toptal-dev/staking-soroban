# Staking Soroban
Allows to stake tokens, unstake, and claim rewards.

## Setup
### Install Soroban
```cargo install --locked --version 0.8.0 soroban-cli```

### Clone Project
```git clone https://github.com/mzaryabrafique/staking-soroban.git```

## Run

### Build
Run this command in staking-soroban directory
```cargo build```

### Run the Tests
```cargo test```

### Build Release for deployment
```cargo build --target wasm32-unknown-unknown --release```

### Deploy Contract:
Deploy contract on the Stellar Futurenet
```
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/staking_soroban.wasm \
    --source REPLACE_HERE_PRIVATE_KEY \
   --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'```


