use crate::tokens::*;
use log::error;
use sea_orm::DbConn;
use std::error::Error;
use std::{future::Future, sync::Arc};

pub fn spawn_sync_task<F, Fut>(db_conn: Arc<DbConn>, sync_fn: F) -> tokio::task::JoinHandle<()>
where
	F: FnOnce(Arc<DbConn>) -> Fut + Send + Sync + 'static,
	Fut: Future<Output = Result<(), Box<dyn Error>>> + Send + 'static,
{
	tokio::spawn(async move {
		sync_fn(db_conn).await.unwrap_or_else(|e| {
			error!("sync task error: {}", e);
		});
	})
}

pub async fn execute_sync_tasks(db_conn: Arc<DbConn>) {
	let sync_tokens_on_ledgers = spawn_sync_task(db_conn, |db_conn| async move { sync_tokens_on_ledgers(&db_conn).await });

	let _ = tokio::join!(sync_tokens_on_ledgers);
}

pub async fn sync_tokens_on_ledgers(db: &DbConn) -> Result<(), Box<dyn Error>> {
	// sync_dragginz(&db).await?;
	// sync_neuron_icp(&db).await?;
	// sync_cketh(&db).await?;
	// sync_ckusdt(&db).await?;
	//61+11=72
	let runes = [
		("CRYPTOBURG_ID_JQNJ_ODIN", "Bitcoin-runes-CRYPTOBURG•ID•JQNJ•ODIN", 8_i16),
		("BITCAT_ID_YRMO_ODIN", "Bitcoin-runes-BITCAT•ID•YRMO•ODIN", 8_i16),
		("FORSETISCN_ID_COIU_ODIN", "Bitcoin-runes-FORSETISCN•ID•COIU•ODIN", 8_i16),
		("BTHACD_ID_FQEE_ODIN", "Bitcoin-runes-BTHACD•ID•FQEE•ODIN", 8_i16),
		("KING_ID_MNJB_ODIN", "Bitcoin-runes-KING•ID•MNJB•ODIN", 8_i16),
		("ODINLOOP_ID_FKJD_ODIN", "Bitcoin-runes-ODINLOOP•ID•FKJD•ODIN", 8_i16),
		("ZYRAS_ID_YQKN_ODIN", "Bitcoin-runes-ZYRAS•ID•YQKN•ODIN", 8_i16),
		("BTCGF_ID_ZJFP_ODIN", "Bitcoin-runes-BTCGF•ID•ZJFP•ODIN", 8_i16),
		("ODINDOG_ID_YTTL_ODIN", "Bitcoin-runes-ODINDOG•ID•YTTL•ODIN", 8_i16),
		("ODINPEPE_ID_HIRM_ODIN", "Bitcoin-runes-ODINPEPE•ID•HIRM•ODIN", 8_i16),
		("ODINPIZZA_ID_DNIO_ODIN", "Bitcoin-runes-ODINPIZZA•ID•DNIO•ODIN", 8_i16),
		("ODINAPE_ID_BVAE_ODIN", "Bitcoin-runes-ODINAPE•ID•BVAE•ODIN", 8_i16),
		("ODINFUN_ID_FDFU_ODIN", "Bitcoin-runes-ODINFUN•ID•FDFU•ODIN", 8_i16),
		("BTCFI_ID_YUKN_ODIN", "Bitcoin-runes-BTCFI•ID•YUKN•ODIN", 8_i16),
		("EGG_ID_QNVR_ODIN", "Bitcoin-runes-EGG•ID•QNVR•ODIN", 8_i16),
		("MX_ID_MEKD_ODIN", "Bitcoin-runes-MX•ID•MEKD•ODIN", 8_i16),
		("ODINSMART_ID_WYUR_ODIN", "Bitcoin-runes-ODINSMART•ID•WYUR•ODIN", 8_i16),
		("MICKEY_ID_LFWD_ODIN", "Bitcoin-runes-MICKEY•ID•LFWD•ODIN", 8_i16),
		("BTFR_ID_TYDK_ODIN", "Bitcoin-runes-BTFR•ID•TYDK•ODIN", 8_i16),
		("SMARTBTC_ID_CHXX_ODIN", "Bitcoin-runes-SMARTBTC•ID•CHXX•ODIN", 8_i16),
		("ASG_ID_XIOQ_ODIN", "Bitcoin-runes-ASG•ID•XIOQ•ODIN", 8_i16),
		("OSK_ID_BBEE_ODIN", "Bitcoin-runes-OSK•ID•BBEE•ODIN", 8_i16),
		("WATTP_ID_INHA_ODIN", "Bitcoin-runes-WATTP•ID•INHA•ODIN", 8_i16),
		("BEARDPALS_ID_ZIAI_ODIN", "Bitcoin-runes-BEARDPALS•ID•ZIAI•ODIN", 8_i16),
		("WETH_ID_BFPX_ODIN", "Bitcoin-runes-WETH•ID•BFPX•ODIN", 8_i16),
		("WCSC_ID_BYUG_ODIN", "Bitcoin-runes-WCSC•ID•BYUG•ODIN", 8_i16),
		("MSTR_ID_MZOU_ODIN", "Bitcoin-runes-MSTR•ID•MZOU•ODIN", 8_i16),
		("SATOSHI_ID_OXTM_ODIN", "Bitcoin-runes-SATOSHI•ID•OXTM•ODIN", 8_i16),
		("BOBO_ID_MNLE_ODIN", "Bitcoin-runes-BOBO•ID•MNLE•ODIN", 8_i16),
		("BITBULL_ID_VCZO_ODIN", "Bitcoin-runes-BITBULL•ID•VCZO•ODIN", 8_i16),
		("ANI_ID_JEYD_ODIN", "Bitcoin-runes-ANI•ID•JEYD•ODIN", 8_i16),
		("ODINGOLD_ID_VACP_ODIN", "Bitcoin-runes-ODINGOLD•ID•VACP•ODIN", 8_i16),
		("UDUCKLING_ID_WHRZ_ODIN", "Bitcoin-runes-UDUCKLING•ID•WHRZ•ODIN", 8_i16),
		("LABUBU_ID_URZS_ODIN", "Bitcoin-runes-LABUBU•ID•URZS•ODIN", 8_i16),
		("BOBAI_ID_XTTH_ODIN", "Bitcoin-runes-BOBAI•ID•XTTH•ODIN", 8_i16),
		("RWAS_ID_LDMY_ODIN", "Bitcoin-runes-RWAS•ID•LDMY•ODIN", 8_i16),
		("YCOIN_ID_LLGI_ODIN", "Bitcoin-runes-YCOIN•ID•LLGI•ODIN", 8_i16),
		("PI_ID_YZHI_ODIN", "Bitcoin-runes-PI•ID•YZHI•ODIN", 8_i16),
		("BTCD_ID_MTTS_ODIN", "Bitcoin-runes-BTCD•ID•MTTS•ODIN", 8_i16),
		("MIMO_ID_JJUP_ODIN", "Bitcoin-runes-MIMO•ID•JJUP•ODIN", 8_i16),
		("COO_ID_HTEX_ODIN", "Bitcoin-runes-COO•ID•HTEX•ODIN", 8_i16),
		("ODINRATS_ID_MPLY_ODIN", "Bitcoin-runes-ODINRATS•ID•MPLY•ODIN", 8_i16),
		("AMERICA_ID_KAXC_ODIN", "Bitcoin-runes-AMERICA•ID•KAXC•ODIN", 8_i16),
		("PIZ_ID_KFPO_ODIN", "Bitcoin-runes-PIZ•ID•KFPO•ODIN", 8_i16),
		("BABYODIN_ID_NWKQ_ODIN", "Bitcoin-runes-BABYODIN•ID•NWKQ•ODIN", 8_i16),
		("WBTC_ID_JIUO_ODIN", "Bitcoin-runes-WBTC•ID•JIUO•ODIN", 8_i16),
		("ITLG_ID_JMDX_ODIN", "Bitcoin-runes-ITLG•ID•JMDX•ODIN", 8_i16),
		("ODINCAT_ID_DHGX_ODIN", "Bitcoin-runes-ODINCAT•ID•DHGX•ODIN", 8_i16),
		("NEWMO_ID_GSXY_ODIN", "Bitcoin-runes-NEWMO•ID•GSXY•ODIN", 8_i16),
		("BABYDOGE_ID_UMUL_ODIN", "Bitcoin-runes-BABYDOGE•ID•UMUL•ODIN", 8_i16),
		("RATEL_ID_CZZA_ODIN", "Bitcoin-runes-RATEL•ID•CZZA•ODIN", 8_i16),
		("DDDD_ID_IXND_ODIN", "Bitcoin-runes-DDDD•ID•IXND•ODIN", 8_i16),
		("WETHO_ID_PWIG_ODIN", "Bitcoin-runes-WETHO•ID•PWIG•ODIN", 8_i16),
		("FIST_ID_OUKV_ODIN", "Bitcoin-runes-FIST•ID•OUKV•ODIN", 8_i16),
		("CRYBABY_ID_UZGV_ODIN", "Bitcoin-runes-CRYBABY•ID•UZGV•ODIN", 8_i16),
		("ODINSTAS_ID_JXGT_ODIN", "Bitcoin-runes-ODINSTAS•ID•JXGT•ODIN", 8_i16),
		("FROLD_ID_SKGQ_ODIN", "Bitcoin-runes-FROLD•ID•SKGQ•ODIN", 8_i16),
		("BITPEPE_ID_GKPC_ODIN", "Bitcoin-runes-BITPEPE•ID•GKPC•ODIN", 8_i16),
		("BTCS_ID_KFBA_ODIN", "Bitcoin-runes-BTCS•ID•KFBA•ODIN", 8_i16),
		("BITDINO_ID_VCOB_ODIN", "Bitcoin-runes-BITDINO•ID•VCOB•ODIN", 8_i16),
		("KEKIUS_ID_EAQQ_ODIN", "Bitcoin-runes-KEKIUS•ID•EAQQ•ODIN", 8_i16),
		// ("BITCAT_ID_EOSE_ODIN", "Bitcoin-runes-BITCAT•ID•EOSE•ODIN", 8_i16),
		// ("RATS_ID_JXIT_ODIN", "Bitcoin-runes-RATS•ID•JXIT•ODIN", 8_i16),
		// ("ICONFUCIUS_ID_RVMN_ODIN", "Bitcoin-runes-ICONFUCIUS•ID•RVMN•ODIN", 8_i16),
		// ("DRAK_ID_HCNC_ODIN", "Bitcoin-runes-DRAK•ID•HCNC•ODIN", 8_i16),
		// ("SPARKS_ID_DTEH_ODIN", "Bitcoin-runes-SPARKS•ID•DTEH•ODIN", 8_i16),
		// ("BITPANDA_ID_UUMF_ODIN", "Bitcoin-runes-BITPANDA•ID•UUMF•ODIN", 8_i16),
		// ("GHOSTNODE_ID_ZVVO_ODIN", "Bitcoin-runes-GHOSTNODE•ID•ZVVO•ODIN", 8_i16),
		// ("GOLDBTC_ID_PGZD_ODIN", "Bitcoin-runes-GOLDBTC•ID•PGZD•ODIN", 8_i16),
		// ("AIDEX_ID_AZNX_ODIN", "Bitcoin-runes-AIDEX•ID•AZNX•ODIN", 8_i16),
		// ("AOT_ID_GRMI_ODIN", "Bitcoin-runes-AOT•ID•GRMI•ODIN", 8_i16),
		// ("FLFWORL_ID_PUFE_ODIN", "Bitcoin-runes-FLFWORL•ID•PUFE•ODIN", 8_i16),
		// ("ODINBOT_ID_GIJQ_ODIN", "Bitcoin-runes-ODINBOT•ID•GIJQ•ODIN", 8_i16),
		// ("ICP_WORLD_COMPUTER", "Bitcoin-runes-ICP•WORLD•COMPUTER", 6_i16),
		// ("MAKE_CRYPTO_FUN_AGAIN", "Bitcoin-runes-MAKE•CRYPTO•FUN•AGAIN", 8_i16),
		// ("PROOF_OF_MEMES", "Bitcoin-runes-PROOF•OF•MEMES", 8_i16),
		// ("PUP_WIF_WOOF_OF_WORK", "Bitcoin-runes-PUP•WIF•WOOF•OF•WORK", 8_i16),
		// ("WBTC_ID_RBTM_ODIN", "Bitcoin-runes-WBTC•ID•RBTM•ODIN", 8_i16),
		// ("BTC_ID_HVUQ_ODIN", "Bitcoin-runes-BTC•ID•HVUQ•ODIN", 8_i16),
		// ("RUNES_ID_AJBS_ODIN", "Bitcoin-runes-RUNES•ID•AJBS•ODIN", 8_i16),
		// ("ODINBTC_ID_ZLOQ_ODIN", "Bitcoin-runes-ODINBTC•ID•ZLOQ•ODIN", 8_i16),
		// ("SUPEREX_ID_OOER_ODIN", "Bitcoin-runes-SUPEREX•ID•OOER•ODIN", 8_i16),
		// ("SOB_ID_YYQH_ODIN", "Bitcoin-runes-SOB•ID•YYQH•ODIN", 8_i16),
		// ("SPONGEBOB_ID_JUXZ_ODIN", "Bitcoin-runes-SPONGEBOB•ID•JUXZ•ODIN", 8_i16),
		// ("BUGATTI_ID_WBTD_ODIN", "Bitcoin-runes-BUGATTI•ID•WBTD•ODIN", 8_i16),
		// ("PIZZA_ID_EVQD_ODIN", "Bitcoin-runes-PIZZA•ID•EVQD•ODIN", 8_i16),
		// ("BAGCAT_ID_JKYH_ODIN", "Bitcoin-runes-BAGCAT•ID•JKYH•ODIN", 8_i16),
		// ("ODIN_ID_SIVA_ODIN", "Bitcoin-runes-ODIN•ID•SIVA•ODIN", 8_i16),
		// ("ETH_ID_XWQV_ODIN", "Bitcoin-runes-ETH•ID•XWQV•ODIN", 8_i16),
		// ("BTCGM_ID_UTSI_ODIN", "Bitcoin-runes-BTCGM•ID•UTSI•ODIN", 8_i16),
		// ("AIZ_ID_AJHX_ODIN", "Bitcoin-runes-AIZ•ID•AJHX•ODIN", 8_i16),
		// ("GDOG_ID_TCCK_ODIN", "Bitcoin-runes-GDOG•ID•TCCK•ODIN", 8_i16),
		// ("GOB_IS_GOB_IS_GOB", "Bitcoin-runes-GOB•IS•GOB•IS•GOB", 2_i16),
		// ("NARRATIVE_ID_GKBG_ODIN", "Bitcoin-runes-NARRATIVE•ID•GKBG•ODIN", 8_i16),
		// ("WATTP_ID_GOIP_ODIN", "Bitcoin-runes-WATTP•ID•GOIP•ODIN", 8_i16),
		// ("WBTCO_ID_JDHI_ODIN", "Bitcoin-runes-WBTCO•ID•JDHI•ODIN", 8_i16),
		// ("OIIAOIIA_ID_SGIG_ODIN", "Bitcoin-runes-OIIAOIIA•ID•SGIG•ODIN", 8_i16),
		// ("OPIZ_ID_MJYV_ODIN", "Bitcoin-runes-OPIZ•ID•MJYV•ODIN", 8_i16),
		// ("DRAKER_ID_MSBJ_ODIN", "Bitcoin-runes-DRAKER•ID•MSBJ•ODIN", 8_i16),
		// ("RWA_ID_JVXY_ODIN", "Bitcoin-runes-RWA•ID•JVXY•ODIN", 8_i16),
		// ("ODINPANDA_ID_DLES_ODIN", "Bitcoin-runes-ODINPANDA•ID•DLES•ODIN", 8_i16),
		// ("BITPNUT_ID_WSYW_ODIN", "Bitcoin-runes-BITPNUT•ID•WSYW•ODIN", 8_i16),
		// ("BUTTERFLY_ID_LRWR_ODIN", "Bitcoin-runes-BUTTERFLY•ID•LRWR•ODIN", 8_i16),
		// ("ODINGLP_ID_MUCC_ODIN", "Bitcoin-runes-ODINGLP•ID•MUCC•ODIN", 8_i16),
		// ("BABYANI_ID_TMKL_ODIN", "Bitcoin-runes-BABYANI•ID•TMKL•ODIN", 8_i16),
		// ("BTL_ID_GUGV_ODINN", "Bitcoin-runes-BTL•ID•GUGV•ODIN", 8_i16),
		// ("GROK_ID_OXWA_ODIN", "Bitcoin-runes-GROK•ID•OXWA•ODIN", 8_i16),
		// ("POPMART_ID_OUQQ_ODIN", "Bitcoin-runes-POPMART•ID•OUQQ•ODIN", 8_i16),
		// ("HORSE_ID_ZBZS_ODIN", "Bitcoin-runes-HORSE•ID•ZBZS•ODIN", 8_i16),
		// ("ODINETX_ID_ACAY_ODIN", "Bitcoin-runes-ODINETX•ID•ACAY•ODIN", 8_i16),
		// ("BITEAGLES_ID_CPLR_ODIN", "Bitcoin-runes-BITEAGLES•ID•CPLR•ODIN", 8_i16),
		// ("SERGEASS_ID_WEZK_ODIN", "Bitcoin-runes-SERGEASS•ID•WEZK•ODIN", 8_i16),
		// ("MAI_ID_LOHB_ODIN", "Bitcoin-runes-MAI•ID•LOHB•ODIN", 8_i16),
		// ("NEZHA_ID_UZWU_ODIN", "Bitcoin-runes-NEZHA•ID•UZWU•ODIN", 8_i16),
		// ("SOON_ID_OREX_ODIN", "Bitcoin-runes-SOON•ID•OREX•ODIN", 8_i16),
		// ("PIXIU_ID_CZCG_ODIN", "Bitcoin-runes-PIXIU•ID•CZCG•ODIN", 8_i16),
		// ("BITFROLD_ID_WWHI_ODIN", "Bitcoin-runes-BITFROLD•ID•WWHI•ODIN", 8_i16),
		// ("TERPLAYER_ID_WOUM_ODIN", "Bitcoin-runes-TERPLAYER•ID•WOUM•ODIN", 8_i16),
		// ("ODINSUN_ID_NJBX_ODIN", "Bitcoin-runes-ODINSUN•ID•NJBX•ODIN", 8_i16),
		// ("WETH_ID_EQXL_ODIN", "Bitcoin-runes-WETH•ID•EQXL•ODIN", 8_i16),
		// ("BTL_ID_PSWX_ODIN", "Bitcoin-runes-BTL•ID•PSWX•ODIN", 8_i16),
		// 11
		("BTCFB_ID_SEOF_ODIN", "Bitcoin-runes-BTCFB•ID•SEOF•ODIN", 8_i16),
		("LTG_ID_UJBI_ODIN", "Bitcoin-runes-LTG•ID•UJBI•ODIN", 8_i16),
		("FWODIN_ID_AJKN_ODIN", "Bitcoin-runes-FWODIN•ID•AJKN•ODIN", 8_i16),
		("ZEBRA_ID_IDFC_ODIN", "Bitcoin-runes-ZEBRA•ID•IDFC•ODIN", 8_i16),
		("GSNAKE_ID_EDLA_ODIN", "Bitcoin-runes-GSNAKE•ID•EDLA•ODIN", 8_i16),
		("PANGU_ID_SUCB_ODIN", "Bitcoin-runes-PANGU•ID•SUCB•ODIN", 8_i16),
		("SCEPTER_ID_TWSK_ODIN", "Bitcoin-runes-SCEPTER•ID•TWSK•ODIN", 8_i16),
		("ODINTRACK_ID_OYYV_ODIN", "Bitcoin-runes-ODINTRACK•ID•OYYV•ODIN", 8_i16),
		("TAURUS_ID_CZTS_ODIN", "Bitcoin-runes-TAURUS•ID•CZTS•ODIN", 8_i16),
		("BTCHACD_ID_AUND_ODIN", "Bitcoin-runes-BTCHACD•ID•AUND•ODIN", 8_i16),
		("CAISHEN_ID_KECF_ODIN", "Bitcoin-runes-CAISHEN•ID•KECF•ODIN", 8_i16),
	];
	for (id, name, decimals) in runes {
		sync_rune(db, id, name, decimals).await?;
	}

	sync_ckbtc(db).await?;
	sync_icp(db).await?;
	sync_rich(db).await?;

	sync_runes_x_bitcoin(db).await?;
	sync_dog_go_to_the_moon(db).await
}
