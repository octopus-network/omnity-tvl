# omnity-tvl

### Create or drop the schema
```bash  
docker compose exec -it postgres bash

psql -U postgres

CREATE DATABASE tvl ENCODING = 'UTF8';

sea-orm-cli migrate up -u postgres://postgres:omnity_go@localhost/tvl
# sea-orm-cli migrate fresh -u postgres://postgres:omnity_go@localhost/tvl
# sea-orm-cli generate entity -u postgres://postgres:omnity_go@localhost:5432/tvl -o src/entities
```

### Build and run

```bash
cargo build --release -p tvl

./target/release/tvl
```

### Config identity
```bash  
export DFX_IDENTITY=$(<./test.pem)
export DATABASE_URL=postgres://postgres:omnity_go@localhost/tvl
export DFX_NETWORK=https://ic0.app
export OMNITY_HUB_CANISTER_ID=7wupf-wiaaa-aaaar-qaeya-cai
export CKBTC_CANISTER_ID=mxzaz-hqaaa-aaaar-qaada-cai
export EICP_HOPE_YOU_GET_RICH=77xez-aaaaa-aaaar-qaezq-cai
export CKETH_CANISTER_ID=ss2fx-dyaaa-aaaar-qacoq-cai
export CKUSDT_CANISTER_ID=cngnf-vqaaa-aaaar-qag4q-cai
export NEURON_CANISTER_ID=buwm7-7yaaa-aaaar-qagva-cai
export DRAGGIN_CANISTER_ID=zfcdd-tqaaa-aaaaq-aaaga-cai
export ICP_CANISTER_ID=ryjl3-tyaaa-aaaaa-aaaba-cai
export ALCHEMY_KEY=
```