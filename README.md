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
export KING_ID_MNJB_ODIN=tmrkv-qiaaa-aaaar-qbt3q-cai
export ODINSMART_ID_WYUR_ODIN=qsan3-xyaaa-aaaar-qbtsq-cai
export NEWMO_ID_GSXY_ODIN=tlqmb-5qaaa-aaaar-qbt3a-cai
export MAI_ID_LOHB_ODIN=6lizf-5aaaa-aaaar-qbsqa-cai
export NEZHA_ID_UZWU_ODIN=qvblp-2aaaa-aaaar-qbtsa-cai
export BTCD_ID_MTTS_ODIN=rkjyy-zyaaa-aaaar-qbtwq-cai
export BTCS_ID_KFBA_ODIN=nhqct-pqaaa-aaaar-qbrxa-cai
export SOON_ID_OREX_ODIN=rdkte-pqaaa-aaaar-qbtxa-cai
export WETHO_ID_PWIG_ODIN=tqvqe-hiaaa-aaaar-qbtzq-cai
export PIXIU_ID_CZCG_ODIN=tfsbj-gaaaa-aaaar-qbt2a-cai
export BITFROLD_ID_WWHI_ODIN=txuwq-kqaaa-aaaar-qbtza-cai
export CRYPTOBURG_ID_JQNJ_ODIN=4sdi3-3iaaa-aaaar-qbvca-cai
export TERPLAYER_ID_WOUM_ODIN=4dsga-paaaa-aaaar-qbs4a-cai
export KEKIUS_ID_EAQQ_ODIN=44bft-ayaaa-aaaar-qbvda-cai
export RWAS_ID_LDMY_ODIN=zs4ip-haaaa-aaaar-qbv7q-cai
export ODINSUN_ID_NJBX_ODIN=yrqbj-tyaaa-aaaar-qbvza-cai
export EGG_ID_QNVR_ODIN=zoys6-qaaaa-aaaar-qbv5q-cai
export ANI_ID_JEYD_ODIN=32gxk-vaaaa-aaaar-qbvtq-cai
export OSK_ID_BBEE_ODIN=oprns-rqaaa-aaaar-qbwaq-cai
export BTFR_ID_TYDK_ODIN=oiqlg-4iaaa-aaaar-qbwaa-cai
export WETH_ID_EQXL_ODIN=2xite-2iaaa-aaaar-qbvua-cai
export ODINLOOP_ID_FKJD_ODIN=y7smb-iiaaa-aaaar-qbvya-cai
export BTCFI_ID_YUKN_ODIN=z46fh-4qaaa-aaaar-qbv6q-cai
export ODINRATS_ID_MPLY_ODIN=kthtr-qyaaa-aaaar-qboba-cai
export WATTP_ID_INHA_ODIN=zh3zc-giaaa-aaaar-qbv4a-cai
export ASG_ID_XIOQ_ODIN=mhlsx-dqaaa-aaaar-qbwmq-cai
export ODINPIZZA_ID_DNIO_ODIN=2foe5-wyaaa-aaaar-qbvxa-cai
export SMARTBTC_ID_CHXX_ODIN=nkfwz-myaaa-aaaar-qbwla-cai
export ITLG_ID_JMDX_ODIN=ykv5m-jaaaa-aaaar-qbv3q-cai
export WCSC_ID_BYUG_ODIN=a3mng-yaaaa-aaaar-qbq6a-cai
export BTL_ID_PSWX_ODIN=nwbmi-3yaaa-aaaar-qbwja-cai
export WBTC_ID_JIUO_ODIN=ywrh5-6aaaa-aaaar-qbvzq-cai

export BTCFB_ID_SEOF_ODIN=ary7y-wqaaa-aaaar-qbxcq-cai
export LTG_ID_UJBI_ODIN=lgng2-3qaaa-aaaar-qbw4q-cai
export FWODIN_ID_AJKN_ODIN=k6etz-vqaaa-aaaar-qbwyq-cai
export ZEBRA_ID_IDFC_ODIN=jstdo-6qaaa-aaaar-qbwsq-cai
export GSNAKE_ID_EDLA_ODIN=iey3f-laaaa-aaaar-qbwxq-cai
export PANGU_ID_SUCB_ODIN=bsuw6-ciaaa-aaaar-qbxea-cai
export SCEPTER_ID_TWSK_ODIN=iw6m4-hqaaa-aaaar-qbwuq-cai
export ODINTRACK_ID_OYYV_ODIN=sqmup-miaaa-aaaar-qarha-cai
export TAURUS_ID_CZTS_ODIN=pl4ca-iqaaa-aaaar-qbwgq-cai
export BTCHACD_ID_AUND_ODIN=2inev-tqaaa-aaaar-qbnoq-cai
export CAISHEN_ID_KECF_ODIN=n7chu-nqaaa-aaaar-qbwiq-cai
```
--set-env-vars BTCFB_ID_SEOF_ODIN=ary7y-wqaaa-aaaar-qbxcq-cai \
--set-env-vars LTG_ID_UJBI_ODIN=lgng2-3qaaa-aaaar-qbw4q-cai \
--set-env-vars FWODIN_ID_AJKN_ODIN=k6etz-vqaaa-aaaar-qbwyq-cai \
--set-env-vars ZEBRA_ID_IDFC_ODIN=jstdo-6qaaa-aaaar-qbwsq-cai \
--set-env-vars GSNAKE_ID_EDLA_ODIN=iey3f-laaaa-aaaar-qbwxq-cai \
--set-env-vars PANGU_ID_SUCB_ODIN=bsuw6-ciaaa-aaaar-qbxea-cai \
--set-env-vars SCEPTER_ID_TWSK_ODIN=iw6m4-hqaaa-aaaar-qbwuq-cai \
--set-env-vars ODINTRACK_ID_OYYV_ODIN=sqmup-miaaa-aaaar-qarha-cai \
--set-env-vars TAURUS_ID_CZTS_ODIN=pl4ca-iqaaa-aaaar-qbwgq-cai \
--set-env-vars BTCHACD_ID_AUND_ODIN=2inev-tqaaa-aaaar-qbnoq-cai \
--set-env-vars CAISHEN_ID_KECF_ODIN=n7chu-nqaaa-aaaar-qbwiq-cai \

```bash 
 --set-env-vars OMNITY_HUB_CANISTER_ID=7wupf-wiaaa-aaaar-qaeya-cai \
  --set-env-vars CKBTC_CANISTER_ID=mxzaz-hqaaa-aaaar-qaada-cai \
  --set-env-vars EICP_HOPE_YOU_GET_RICH=77xez-aaaaa-aaaar-qaezq-cai \
  --set-env-vars CKETH_CANISTER_ID=ss2fx-dyaaa-aaaar-qacoq-cai \
  --set-env-vars CKUSDT_CANISTER_ID=cngnf-vqaaa-aaaar-qag4q-cai \
  --set-env-vars NEURON_CANISTER_ID=buwm7-7yaaa-aaaar-qagva-cai \
  --set-env-vars DRAGGIN_CANISTER_ID=zfcdd-tqaaa-aaaaq-aaaga-cai \
  --set-env-vars ICP_CANISTER_ID=ryjl3-tyaaa-aaaaa-aaaba-cai \
  --set-env-vars ODINAPE_ID_BVAE_ODIN=26jmx-laaaa-aaaar-qaqqq-cai \
  --set-env-vars ODINDOG_ID_YTTL_ODIN=eazb6-tqaaa-aaaar-qan2a-cai \
  --set-env-vars ODINGOLD_ID_VACP_ODIN=6pkg6-jaaaa-aaaar-qapna-cai \
  --set-env-vars SATOSHI_ID_OXTM_ODIN=sf5ra-laaaa-aaaar-qaoba-cai \
  --set-env-vars ODINSTAS_ID_JXGT_ODIN=sxns3-bqaaa-aaaar-qarhq-cai \
  --set-env-vars BITCAT_ID_EOSE_ODIN=7mgpy-5yaaa-aaaar-qaplq-cai \
  --set-env-vars ODINCAT_ID_DHGX_ODIN=sl74i-qqaaa-aaaar-qaoaa-cai \
  --set-env-vars FORSETISCN_ID_COIU_ODIN=tta5j-yqaaa-aaaar-qarbq-cai \
  --set-env-vars PI_ID_YZHI_ODIN=5n73b-zqaaa-aaaar-qapga-cai \
  --set-env-vars RATS_ID_JXIT_ODIN=wpnto-vyaaa-aaaar-qao3q-cai \
  --set-env-vars ICONFUCIUS_ID_RVMN_ODIN=5kijx-siaaa-aaaar-qaqda-cai \
  --set-env-vars DRAK_ID_HCNC_ODIN=qdtxp-eqaaa-aaaar-qarjq-cai \
  --set-env-vars SPARKS_ID_DTEH_ODIN=fkwde-raaaa-aaaar-qan5a-cai \
  --set-env-vars BITPANDA_ID_UUMF_ODIN=7xvh7-baaaa-aaaar-qaqmq-cai \
  --set-env-vars GHOSTNODE_ID_ZVVO_ODIN=sx3gz-hqaaa-aaaar-qaoca-cai \
  --set-env-vars BITCAT_ID_YRMO_ODIN=xlwi6-kyaaa-aaaar-qarya-cai \
  --set-env-vars GOLDBTC_ID_PGZD_ODIN=xfufw-riaaa-aaaar-qarza-cai \
  --set-env-vars AIDEX_ID_AZNX_ODIN=vno2t-diaaa-aaaar-qarva-cai \
  --set-env-vars AOT_ID_GRMI_ODIN=vrkac-uiaaa-aaaar-qarxa-cai \
  --set-env-vars OMNITY_CUSTOMS_BITCOIN_CANISTER_ID=7rvjr-3qaaa-aaaar-qaeyq-cai \
  --set-env-vars ODINPEPE_ID_HIRM_ODIN=2zikd-gyaaa-aaaar-qaqqa-cai \
  --set-env-vars FLFWORL_ID_PUFE_ODIN=w5jqy-6iaaa-aaaar-qbl5q-cai \
  --set-env-vars UDUCKLING_ID_WHRZ_ODIN=wtl5q-fyaaa-aaaar-qbl4q-cai \
  --set-env-vars ODINBOT_ID_GIJQ_ODIN=wz7po-liaaa-aaaar-qbmaa-cai \
  --set-env-vars BITBULL_ID_VCZO_ODIN=wpphb-syaaa-aaaar-qbl6q-cai \
  --set-env-vars ICP_WORLD_COMPUTER=gbavh-xyaaa-aaaar-qanxq-cai \
  --set-env-vars MAKE_CRYPTO_FUN_AGAIN=ej2kc-fyaaa-aaaar-qan3q-cai \
  --set-env-vars PROOF_OF_MEMES=eo3mw-iaaaa-aaaar-qan3a-cai \
  --set-env-vars PUP_WIF_WOOF_OF_WORK=ehyhk-6iaaa-aaaar-qan2q-cai \
  --set-env-vars BOBAI_ID_XTTH_ODIN=6zy26-xqaaa-aaaar-qbnwq-cai \
  --set-env-vars BTCGF_ID_ZJFP_ODIN=7ptcv-caaaa-aaaar-qbntq-cai \
  --set-env-vars BTHACD_ID_FQEE_ODIN=22ltm-7aaaa-aaaar-qbnnq-cai \
  --set-env-vars WBTC_ID_RBTM_ODIN=6c5g3-niaaa-aaaar-qbnua-cai \
  --set-env-vars BTC_ID_HVUQ_ODIN=6q3rc-byaaa-aaaar-qbnxa-cai \
  --set-env-vars COO_ID_HTEX_ODIN=2pmcb-6iaaa-aaaar-qbnoa-cai \
  --set-env-vars RUNES_ID_AJBS_ODIN=u3tw2-qyaaa-aaaar-qaovq-cai \
  --set-env-vars ODINBTC_ID_ZLOQ_ODIN=wbnkj-jiaaa-aaaar-qbl7q-cai \
  --set-env-vars SUPEREX_ID_OOER_ODIN=2mzpm-bqaaa-aaaar-qapwa-cai \
  --set-env-vars SOB_ID_YYQH_ODIN=q4uur-kiaaa-aaaar-qbmwa-cai \
  --set-env-vars SPONGEBOB_ID_JUXZ_ODIN=xmb2i-baaaa-aaaar-qao5a-cai \
  --set-env-vars BUGATTI_ID_WBTD_ODIN=xcdxa-2qaaa-aaaar-qao4a-cai \
  --set-env-vars PIZZA_ID_EVQD_ODIN=wimv2-yaaaa-aaaar-qao3a-cai \
  --set-env-vars BAGCAT_ID_JKYH_ODIN=xzglf-aiaaa-aaaar-qao6q-cai \
  --set-env-vars MSTR_ID_MZOU_ODIN=5som5-riaaa-aaaar-qbn4a-cai \
  --set-env-vars ODIN_ID_SIVA_ODIN=6tyin-yaaaa-aaaar-qaqkq-cai\
  --set-env-vars LABUBU_ID_URZS_ODIN=5vpkj-4qaaa-aaaar-qbn4q-cai \
  --set-env-vars ETH_ID_XWQV_ODIN=oxv4x-vqaaa-aaaar-qbo2q-cai \
  --set-env-vars BOBO_ID_MNLE_ODIN=oftlo-zaaaa-aaaar-qbozq-cai \
  --set-env-vars BTCGM_ID_UTSI_ODIN=o6wxl-dyaaa-aaaar-qbo3a-cai \
  --set-env-vars AIZ_ID_AJHX_ODIN=ozxr7-oaaaa-aaaar-qbo3q-cai \
  --set-env-vars GDOG_ID_TCCK_ODIN=ihzwf-vyaaa-aaaar-qbopa-cai \
  --set-env-vars GOB_IS_GOB_IS_GOB=re5bs-eiaaa-aaaar-qbmsa-cai \
  --set-env-vars NARRATIVE_ID_GKBG_ODIN=k2eyn-gqaaa-aaaar-qboaq-cai \
  --set-env-vars WETH_ID_BFPX_ODIN=myofg-kiaaa-aaaar-qbowa-cai \
  --set-env-vars EICP_RUNES_X_BITCOIN=2tkm7-oiaaa-aaaar-qaija-cai \
  --set-env-vars EICP_DOG_GO_TO_THE_MOON=fmazi-nqaaa-aaaar-qagmq-cai \
  --set-env-vars ODINFUN_ID_FDFU_ODIN=notjp-zyaaa-aaaar-qbrwq-cai \
  --set-env-vars WATTP_ID_GOIP_ODIN=nsxt6-oyaaa-aaaar-qbruq-cai \
  --set-env-vars ZYRAS_ID_YQKN_ODIN=lx4ib-pyaaa-aaaar-qbrcq-cai \
  --set-env-vars MX_ID_MEKD_ODIN=47w4r-yaaaa-aaaar-qbs6a-cai \
  --set-env-vars MICKEY_ID_LFWD_ODIN=xuh7c-caaaa-aaaar-qbtca-cai \
  --set-env-vars WBTCO_ID_JDHI_ODIN=v45ah-qaaaa-aaaar-qbtoa-cai \
  --set-env-vars OIIAOIIA_ID_SGIG_ODIN=6clsz-liaaa-aaaar-qbsrq-cai \
  --set-env-vars OPIZ_ID_MJYV_ODIN=zemaa-6qaaa-aaaar-qbsba-cai \
  --set-env-vars RATEL_ID_CZZA_ODIN=kiu3w-maaaa-aaaar-qbrga-cai \
  --set-env-vars BITDINO_ID_VCOB_ODIN=aol4l-ziaaa-aaaar-qbq5q-cai \
  --set-env-vars DRAKER_ID_MSBJ_ODIN=isitk-sqaaa-aaaar-qbrja-cai \
  --set-env-vars PIZ_ID_KFPO_ODIN=iaoet-6aaaa-aaaar-qbrka-cai \
  --set-env-vars RWA_ID_JVXY_ODIN=lf27y-diaaa-aaaar-qbrbq-cai \
  --set-env-vars ODINPANDA_ID_DLES_ODIN=vj2rk-riaaa-aaaar-qbtnq-cai \
  --set-env-vars BITPNUT_ID_WSYW_ODIN=nvwvk-daaaa-aaaar-qbrua-cai \
  --set-env-vars DDDD_ID_IXND_ODIN=wzj3m-niaaa-aaaar-qbtfq-cai \
  --set-env-vars CRYBABY_ID_UZGV_ODIN=m7zxq-bqaaa-aaaar-qbrta-cai \
  --set-env-vars BEARDPALS_ID_ZIAI_ODIN=ihpch-tyaaa-aaaar-qbrkq-cai \
  --set-env-vars YCOIN_ID_LLGI_ODIN=x2fsk-zqaaa-aaaar-qbtda-cai \
  --set-env-vars FIST_ID_OUKV_ODIN=y4fvd-qqaaa-aaaar-qbsfa-cai \
  --set-env-vars BUTTERFLY_ID_LRWR_ODIN=3zroi-nyaaa-aaaar-qbsoq-cai \
  --set-env-vars AMERICA_ID_KAXC_ODIN=kuqbh-3aaaa-aaaar-qbrea-cai \
  --set-env-vars MIMO_ID_JJUP_ODIN=xbaop-diaaa-aaaar-qbtbq-cai \
  --set-env-vars ODINGLP_ID_MUCC_ODIN=ctwsd-kaaaa-aaaar-qbqsa-cai \
  --set-env-vars BABYODIN_ID_NWKQ_ODIN=n4v6w-viaaa-aaaar-qbrvq-cai \
  --set-env-vars BABYANI_ID_TMKL_ODIN=u7rjb-eyaaa-aaaar-qbtiq-cai \
  --set-env-vars BABYDOGE_ID_UMUL_ODIN=uyqpv-jaaaa-aaaar-qbtia-cai \
  --set-env-vars BTL_ID_GUGV_ODINN=xidft-vaaaa-aaaar-qbtaa-cai \
  --set-env-vars GROK_ID_OXWA_ODIN=pil3c-qiaaa-aaaar-qbr3q-cai \
  --set-env-vars FROLD_ID_SKGQ_ODIN=w6i5y-aqaaa-aaaar-qbtfa-cai \
  --set-env-vars BITPEPE_ID_GKPC_ODIN=dx35r-taaaa-aaaar-qbqua-cai \
  --set-env-vars POPMART_ID_OUQQ_ODIN=mrno2-4aaaa-aaaar-qboxq-cai \
  --set-env-vars HORSE_ID_ZBZS_ODIN=qjfr6-naaaa-aaaar-qbtqa-cai \
  --set-env-vars ODINETX_ID_ACAY_ODIN=qkq4t-syaaa-aaaar-qaria-cai \
  --set-env-vars BITEAGLES_ID_CPLR_ODIN=wxlwe-wyaaa-aaaar-qbteq-cai \
  --set-env-vars SERGEASS_ID_WEZK_ODIN=6to4p-6aaaa-aaaar-qappa-cai \
  --set-env-vars KING_ID_MNJB_ODIN=tmrkv-qiaaa-aaaar-qbt3q-cai \
  --set-env-vars ODINSMART_ID_WYUR_ODIN=qsan3-xyaaa-aaaar-qbtsq-cai \
  --set-env-vars NEWMO_ID_GSXY_ODIN=tlqmb-5qaaa-aaaar-qbt3a-cai \
  --set-env-vars MAI_ID_LOHB_ODIN=6lizf-5aaaa-aaaar-qbsqa-cai \
  --set-env-vars NEZHA_ID_UZWU_ODIN=qvblp-2aaaa-aaaar-qbtsa-cai \
  --set-env-vars BTCD_ID_MTTS_ODIN=rkjyy-zyaaa-aaaar-qbtwq-cai \
  --set-env-vars BTCS_ID_KFBA_ODIN=nhqct-pqaaa-aaaar-qbrxa-cai \
  --set-env-vars SOON_ID_OREX_ODIN=rdkte-pqaaa-aaaar-qbtxa-cai \
  --set-env-vars WETHO_ID_PWIG_ODIN=tqvqe-hiaaa-aaaar-qbtzq-cai \
  --set-env-vars PIXIU_ID_CZCG_ODIN=tfsbj-gaaaa-aaaar-qbt2a-cai \
  --set-env-vars BITFROLD_ID_WWHI_ODIN=txuwq-kqaaa-aaaar-qbtza-cai \
  --set-env-vars CRYPTOBURG_ID_JQNJ_ODIN=4sdi3-3iaaa-aaaar-qbvca-cai \
  --set-env-vars TERPLAYER_ID_WOUM_ODIN=4dsga-paaaa-aaaar-qbs4a-cai \
  --set-env-vars KEKIUS_ID_EAQQ_ODIN=44bft-ayaaa-aaaar-qbvda-cai \
  --set-env-vars RWAS_ID_LDMY_ODIN=zs4ip-haaaa-aaaar-qbv7q-cai \
  --set-env-vars ODINSUN_ID_NJBX_ODIN=yrqbj-tyaaa-aaaar-qbvza-cai \
  --set-env-vars EGG_ID_QNVR_ODIN=zoys6-qaaaa-aaaar-qbv5q-cai \
  --set-env-vars ANI_ID_JEYD_ODIN=32gxk-vaaaa-aaaar-qbvtq-cai \
  --set-env-vars OSK_ID_BBEE_ODIN=oprns-rqaaa-aaaar-qbwaq-cai \
  --set-env-vars BTFR_ID_TYDK_ODIN=oiqlg-4iaaa-aaaar-qbwaa-cai \
  --set-env-vars WETH_ID_EQXL_ODIN=2xite-2iaaa-aaaar-qbvua-cai \
--set-env-vars ODINLOOP_ID_FKJD_ODIN=y7smb-iiaaa-aaaar-qbvya-cai \
--set-env-vars BTCFI_ID_YUKN_ODIN=z46fh-4qaaa-aaaar-qbv6q-cai \
--set-env-vars ODINRATS_ID_MPLY_ODIN=kthtr-qyaaa-aaaar-qboba-cai \
--set-env-vars WATTP_ID_INHA_ODIN=zh3zc-giaaa-aaaar-qbv4a-cai \
--set-env-vars ASG_ID_XIOQ_ODIN=mhlsx-dqaaa-aaaar-qbwmq-cai \
--set-env-vars ODINPIZZA_ID_DNIO_ODIN=2foe5-wyaaa-aaaar-qbvxa-cai \
--set-env-vars SMARTBTC_ID_CHXX_ODIN=nkfwz-myaaa-aaaar-qbwla-cai \
--set-env-vars ITLG_ID_JMDX_ODIN=ykv5m-jaaaa-aaaar-qbv3q-cai \
--set-env-vars WCSC_ID_BYUG_ODIN=a3mng-yaaaa-aaaar-qbq6a-cai \
--set-env-vars BTL_ID_PSWX_ODIN=nwbmi-3yaaa-aaaar-qbwja-cai \
--set-env-vars WBTC_ID_JIUO_ODIN=ywrh5-6aaaa-aaaar-qbvzq-cai \
```






