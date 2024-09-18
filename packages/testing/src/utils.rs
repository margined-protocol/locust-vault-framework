use osmosis_test_tube::{OsmosisTestApp, RunnerError, SigningAccount, Wasm};
use std::{fmt::Display, fs, path::PathBuf};

pub fn wasm_file(contract_name: &str) -> Result<String, String> {
    let snaked_name = contract_name.replace('-', "_");
    let target_path = PathBuf::from(format!(
        "../../target/wasm32-unknown-unknown/release/{}.wasm",
        snaked_name
    ));

    if target_path.exists() {
        Ok(target_path.to_string_lossy().into_owned())
    } else {
        let arch = std::env::consts::ARCH;
        let artifacts_dir =
            std::env::var("ARTIFACTS_DIR_PATH").unwrap_or_else(|_| "artifacts".to_owned());
        let fallback_path = PathBuf::from(format!(
            "../{}/{}-{}.wasm",
            artifacts_dir, snaked_name, arch
        ));

        if fallback_path.exists() {
            Ok(fallback_path.to_string_lossy().into_owned())
        } else {
            Err(format!(
                "Wasm file for contract '{}' not found in release or artifacts directory.",
                contract_name
            ))
        }
    }
}

pub fn store_code(
    wasm: &Wasm<OsmosisTestApp>,
    owner: &SigningAccount,
    contract_name: &str,
) -> Result<u64, String> {
    let wasm_byte_code_path = wasm_file(contract_name)?;

    let wasm_byte_code =
        fs::read(wasm_byte_code_path).map_err(|e| format!("Failed to read Wasm file: {}", e))?;

    wasm.store_code(&wasm_byte_code, None, owner)
        .map(|res| res.data.code_id)
        .map_err(|e| format!("Failed to store code: {}", e))
}

pub fn assert_err(actual: RunnerError, expected: impl Display) {
    match actual {
        RunnerError::ExecuteError { msg } => assert!(msg.contains(&expected.to_string())),
        RunnerError::QueryError { msg } => assert!(msg.contains(&expected.to_string())),
        _ => panic!("Unhandled error"),
    }
}
