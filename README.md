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
export SPONGEBOB_ID_JUXZ_ODIN=xmb2i-baaaa-aaaar-qao5a-cai
export BUGATTI_ID_WBTD_ODIN=xcdxa-2qaaa-aaaar-qao4a-cai
export PIZZA_ID_EVQD_ODIN=wimv2-yaaaa-aaaar-qao3a-cai
export BAGCAT_ID_JKYH_ODIN=xzglf-aiaaa-aaaar-qao6q-cai
export MSTR_ID_MZOU_ODIN=5som5-riaaa-aaaar-qbn4a-cai
export ODIN_ID_SIVA_ODIN=6tyin-yaaaa-aaaar-qaqkq-cai
export LABUBU_ID_URZS_ODIN=5vpkj-4qaaa-aaaar-qbn4q-cai
export ETH_ID_XWQV_ODIN=oxv4x-vqaaa-aaaar-qbo2q-cai
export BOBO_ID_MNLE_ODIN=oftlo-zaaaa-aaaar-qbozq-cai
export BTCGM_ID_UTSI_ODIN=o6wxl-dyaaa-aaaar-qbo3a-cai
export AIZ_ID_AJHX_ODIN=ozxr7-oaaaa-aaaar-qbo3q-cai
export GDOG_ID_TCCK_ODIN=ihzwf-vyaaa-aaaar-qbopa-cai
export GOB_IS_GOB_IS_GOB=re5bs-eiaaa-aaaar-qbmsa-cai
export NARRATIVE_ID_GKBG_ODIN=k2eyn-gqaaa-aaaar-qboaq-cai
export WETH_ID_BFPX_ODIN=myofg-kiaaa-aaaar-qbowa-cai

export EICP_RUNES_X_BITCOIN=2tkm7-oiaaa-aaaar-qaija-cai
export EICP_DOG_GO_TO_THE_MOON=fmazi-nqaaa-aaaar-qagmq-cai

export ODINFUN_ID_FDFU_ODIN=notjp-zyaaa-aaaar-qbrwq-cai
export WATTP_ID_GOIP_ODIN=nsxt6-oyaaa-aaaar-qbruq-cai
export ZYRAS_ID_YQKN_ODIN=lx4ib-pyaaa-aaaar-qbrcq-cai
export MX_ID_MEKD_ODIN=47w4r-yaaaa-aaaar-qbs6a-cai
export MICKEY_ID_LFWD_ODIN=xuh7c-caaaa-aaaar-qbtca-cai
export WBTCO_ID_JDHI_ODIN=v45ah-qaaaa-aaaar-qbtoa-cai
export OIIAOIIA_ID_SGIG_ODIN=6clsz-liaaa-aaaar-qbsrq-cai
export OPIZ_ID_MJYV_ODIN=zemaa-6qaaa-aaaar-qbsba-cai
export RATEL_ID_CZZA_ODIN=kiu3w-maaaa-aaaar-qbrga-cai
export BITDINO_ID_VCOB_ODIN=aol4l-ziaaa-aaaar-qbq5q-cai
export DRAKER_ID_MSBJ_ODIN=isitk-sqaaa-aaaar-qbrja-cai
export PIZ_ID_KFPO_ODIN=iaoet-6aaaa-aaaar-qbrka-cai
export RWA_ID_JVXY_ODIN=lf27y-diaaa-aaaar-qbrbq-cai
export ODINPANDA_ID_DLES_ODIN=vj2rk-riaaa-aaaar-qbtnq-cai
export BITPNUT_ID_WSYW_ODIN=nvwvk-daaaa-aaaar-qbrua-cai
export DDDD_ID_IXND_ODIN=wzj3m-niaaa-aaaar-qbtfq-cai
export CRYBABY_ID_UZGV_ODIN=m7zxq-bqaaa-aaaar-qbrta-cai
export BEARDPALS_ID_ZIAI_ODIN=ihpch-tyaaa-aaaar-qbrkq-cai
export YCOIN_ID_LLGI_ODIN=x2fsk-zqaaa-aaaar-qbtda-cai
export FIST_ID_OUKV_ODIN=y4fvd-qqaaa-aaaar-qbsfa-cai
export BUTTERFLY_ID_LRWR_ODIN=3zroi-nyaaa-aaaar-qbsoq-cai
export AMERICA_ID_KAXC_ODIN=kuqbh-3aaaa-aaaar-qbrea-cai
export MIMO_ID_JJUP_ODIN=xbaop-diaaa-aaaar-qbtbq-cai
export ODINGLP_ID_MUCC_ODIN=ctwsd-kaaaa-aaaar-qbqsa-cai
export BABYODIN_ID_NWKQ_ODIN=n4v6w-viaaa-aaaar-qbrvq-cai
export BABYANI_ID_TMKL_ODIN=u7rjb-eyaaa-aaaar-qbtiq-cai
export BABYDOGE_ID_UMUL_ODIN=uyqpv-jaaaa-aaaar-qbtia-cai
export BTL_ID_GUGV_ODINN=xidft-vaaaa-aaaar-qbtaa-cai
export GROK_ID_OXWA_ODIN=pil3c-qiaaa-aaaar-qbr3q-cai
export FROLD_ID_SKGQ_ODIN=w6i5y-aqaaa-aaaar-qbtfa-cai
export BITPEPE_ID_GKPC_ODIN=dx35r-taaaa-aaaar-qbqua-cai
export POPMART_ID_OUQQ_ODIN=mrno2-4aaaa-aaaar-qboxq-cai
export HORSE_ID_ZBZS_ODIN=qjfr6-naaaa-aaaar-qbtqa-cai
export ODINETX_ID_ACAY_ODIN=qkq4t-syaaa-aaaar-qaria-cai
export BITEAGLES_ID_CPLR_ODIN=wxlwe-wyaaa-aaaar-qbteq-cai
export SERGEASS_ID_WEZK_ODIN=6to4p-6aaaa-aaaar-qappa-cai
```
