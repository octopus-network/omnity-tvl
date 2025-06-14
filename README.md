# omnity-tvl

### Create or drop the schema
```bash  
docker compose exec -it postgres bash

psql -U postgres

CREATE DATABASE tvl ENCODING = 'UTF8';

sea-orm-cli migrate up -u postgres://postgres:omnity_go@localhost/tvl
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
export OMNITY_CUSTOMS_BITCOIN_CANISTER_ID=7rvjr-3qaaa-aaaar-qaeyq-cai
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
export ODINPEPE_ID_HIRM_ODIN=2zikd-gyaaa-aaaar-qaqqa-cai
export FLFWORL_ID_PUFE_ODIN=w5jqy-6iaaa-aaaar-qbl5q-cai
export UDUCKLING_ID_WHRZ_ODIN=wtl5q-fyaaa-aaaar-qbl4q-cai
export ODINBOT_ID_GIJQ_ODIN=wz7po-liaaa-aaaar-qbmaa-cai
export BITBULL_ID_VCZO_ODIN=wpphb-syaaa-aaaar-qbl6q-cai
export ICP_WORLD_COMPUTER=gbavh-xyaaa-aaaar-qanxq-cai
export MAKE_CRYPTO_FUN_AGAIN=ej2kc-fyaaa-aaaar-qan3q-cai
export PROOF_OF_MEMES=eo3mw-iaaaa-aaaar-qan3a-cai
export PUP_WIF_WOOF_OF_WORK=ehyhk-6iaaa-aaaar-qan2q-cai

export BOBAI_ID_XTTH_ODIN=6zy26-xqaaa-aaaar-qbnwq-cai
export BTCGF_ID_ZJFP_ODIN=7ptcv-caaaa-aaaar-qbntq-cai
export BTHACD_ID_FQEE_ODIN=22ltm-7aaaa-aaaar-qbnnq-cai
export WBTC_ID_RBTM_ODIN=6c5g3-niaaa-aaaar-qbnua-cai
export BTC_ID_HVUQ_ODIN=6q3rc-byaaa-aaaar-qbnxa-cai
export COO_ID_HTEX_ODIN=2pmcb-6iaaa-aaaar-qbnoa-cai
export RUNES_ID_AJBS_ODIN=u3tw2-qyaaa-aaaar-qaovq-cai
export ODINBTC_ID_ZLOQ_ODIN=wbnkj-jiaaa-aaaar-qbl7q-cai
export SUPEREX_ID_OOER_ODIN=2mzpm-bqaaa-aaaar-qapwa-cai
export SOB_ID_YYQH_ODIN=q4uur-kiaaa-aaaar-qbmwa-cai
```