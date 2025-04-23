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

export ODINAPE_ID_BVAE_ODIN=26jmx-laaaa-aaaar-qaqqq-cai
export ODINDOG_ID_YTTL_ODIN=eazb6-tqaaa-aaaar-qan2a-cai
export ODINGOLD_ID_VACP_ODIN=6pkg6-jaaaa-aaaar-qapna-cai
export SATOSHI_ID_OXTM_ODIN=sf5ra-laaaa-aaaar-qaoba-cai
export ODINSTAS_ID_JXGT_ODIN=sxns3-bqaaa-aaaar-qarhq-cai
export BITCAT_ID_EOSE_ODIN=7mgpy-5yaaa-aaaar-qaplq-cai
export ODINCAT_ID_DHGX_ODIN=sl74i-qqaaa-aaaar-qaoaa-cai
export FORSETISCN_ID_COIU_ODIN=tta5j-yqaaa-aaaar-qarbq-cai
export PI_ID_YZHI_ODIN=5n73b-zqaaa-aaaar-qapga-cai
export RATS_ID_JXIT_ODIN=wpnto-vyaaa-aaaar-qao3q-cai
export ICONFUCIUS_ID_RVMN_ODIN=5kijx-siaaa-aaaar-qaqda-cai
export DRAK_ID_HCNC_ODIN=qdtxp-eqaaa-aaaar-qarjq-cai
export SPARKS_ID_DTEH_ODIN=fkwde-raaaa-aaaar-qan5a-cai
export BITPANDA_ID_UUMF_ODIN=7xvh7-baaaa-aaaar-qaqmq-cai
export GHOSTNODE_ID_ZVVO_ODIN=sx3gz-hqaaa-aaaar-qaoca-cai
export BITCAT_ID_YRMO_ODIN=xlwi6-kyaaa-aaaar-qarya-cai
export GOLDBTC_ID_PGZD_ODIN=xfufw-riaaa-aaaar-qarza-cai
export AIDEX_ID_AZNX_ODIN=vno2t-diaaa-aaaar-qarva-cai
export AOT_ID_GRMI_ODIN=vrkac-uiaaa-aaaar-qarxa-cai
```