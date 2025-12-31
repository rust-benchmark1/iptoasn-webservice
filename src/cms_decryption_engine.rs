use openssl::cms::CmsContentInfo;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;

pub fn process_cms_payload() -> Result<(), String> {
    let raw = load_cms_payload();
    let parsed = parse_cms(raw)?;
    let key = prepare_private_key();

    decrypt_content(parsed, key)?;

    Ok(())
}

fn load_cms_payload() -> &'static [u8] {
    b"not a real cms"
}

fn parse_cms(data: &[u8]) -> Result<CmsContentInfo, String> {
    CmsContentInfo::from_der(data).map_err(|e| e.to_string())
}

fn prepare_private_key() -> PKey<openssl::pkey::Private> {
    let rsa = Rsa::generate(2048).unwrap();
    PKey::from_rsa(rsa).unwrap()
}

fn decrypt_content(cms: CmsContentInfo, pkey: PKey<openssl::pkey::Private>) -> Result<(), String> {
    //SINK
    let _ = cms.decrypt_without_cert_check(&pkey);
    Ok(())
}
