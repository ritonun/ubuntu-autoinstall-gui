pub fn validate_locale(locale: &str) -> Result<(), String> {
    let locales = [
        "C",
        "C.utf8",
        "en_AG",
        "en_AG.utf8",
        "en_AU.utf8",
        "en_BW.utf8",
        "en_CA.utf8",
        "en_DK.utf8",
        "en_GB.utf8",
        "en_HK.utf8",
        "en_IE.utf8",
        "en_IL",
        "en_IL.utf8",
        "en_IN",
        "en_IN.utf8",
        "en_NG",
        "en_NG.utf8",
        "en_NZ.utf8",
        "en_PH.utf8",
        "en_SG.utf8",
        "en_US.utf8",
        "en_ZA.utf8",
        "en_ZM",
        "en_ZM.utf8",
        "en_ZW.utf8",
        "POSIX",
    ];

    if !locales.contains(&locale) {
        Err("Locale is not a possible value".to_string())
    }

    Ok(())
}
