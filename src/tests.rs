use super::*;
use std::path::{Path, PathBuf};
use std::io::{Write, Error};
use std::panic;

struct ScopedFile {
    fq_filename: PathBuf,
}

impl ScopedFile {
    pub fn new(fq_filename: PathBuf) -> Result<ScopedFile, Error> {
        ScopedFile::from_string(fq_filename, "")
    }

    pub fn from_string(fq_filename: PathBuf, file_contents: &str) -> Result<ScopedFile, Error> {
        std::fs::File::create(&fq_filename)?.write_all(file_contents.as_bytes())?;
        Ok(ScopedFile { fq_filename: fq_filename, })
    }
}

impl Drop for ScopedFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.fq_filename);
    }
}

#[test]
fn app_call_with_no_params_succeeds() {
    //create a fake lld so the app does not fail to run due to 'ld not found' error
    const LLD_FILENAME_KEY: &'static str = "test-ld.lld";
    const LLD_FILENAME: &'static str = "LLD_FILENAME";
    std::env::set_var(LLD_FILENAME_KEY, LLD_FILENAME);

    const LLD_PATH_ENV_KEY: &'static str = "LLD_PATH";
    let temp_dir = std::env::temp_dir();
    std::env::set_var(LLD_PATH_ENV_KEY, temp_dir.to_str().expect("Self-created environment variable not found!"));

    let f_handle = ScopedFile::new(Path::new(&(get_linker_uri())).to_path_buf());
    //Rust currently has no way to set the executable bit on a file, so the following will panic (permission denied)
    //when libMain() tries to run test-ld.lld.
    lib_main(vec!["a", "b", "-o", "d"])}

//#[test]
//fn get_output_filename_extracts_output_filename() {
//    let args = ["-o", "testOutputFilename"].to_vec();
//    let result = args.get_output_filename();
//    assert!(result == Some("testOutputFilename"));
//}
//
//#[test]
//fn get_output_filename_returns_none_when_given_insufficient_output_args() {
//    let args = vec!["-o"];
//    let result = args.get_output_filename();
//    assert!(result == None);
//}
//
//#[test]
//fn get_output_filename_returns_none_when_given_no_output_args() {
//    let args = vec!["-anything_but_o"];
//    let result = args.get_output_filename();
//    assert!(result == None);
//}
//
//#[test]
//fn remove_output_filename_switch_and_param() {
//    let args = vec!["-a", "-o", "remove_me", "-z"];
//    let result = args.remove_output_filename_switches_and_params();
//    assert!(!result.iter().any(|&el| el == "-o" || el == "remove_me"));
//    assert!(result.len() == 2);
//}
//
//#[test]
//fn remove_output_filename_switch_and_dash_o_param() {
//    let args = vec!["-a", "-o", "-o", "-z"];
//    let result = args.remove_output_filename_switches_and_params();
//    assert!(!result.iter().any(|&el| el == "-o" || el == "remove_me"));
//    assert!(result.len() == 2);
//}
//
//#[test]
//fn remove_output_filename_switch_and_two_dash_o_params() {
//    let args = vec!["-a", "-o", "remove_me", "-o", "-z"];
//    let result = args.remove_output_filename_switches_and_params();
//    assert!(!result.iter().any(|&el| el == "-o" || el == "remove_me"));
//    assert!(result.len() == 1);
//}
//
//#[test]
//fn remove_output_filename_switch_no_param() {
//    let args = vec!["-a", "-o"];
//    let result = args.remove_output_filename_switches_and_params();
//    assert!(!result.iter().any(|&el| el == "-o" || el == "remove_me"));
//    assert!(result.len() == 1);
//}
//
//#[test]
//fn remove_nostartfiles_no_defaultlibs_switches() {
//    let args = vec!["-a", "-nostartfiles", "-nostartfiles", "-y", "-z"];
//    let result = args.remove_nostartfiles_nodefaultlibs_switches();
//    assert!(!result.iter().any(|&el| el == "-nostartfiles"));
//    assert!(result.len() == 3);
//}
//
//#[test]
//fn remove_wl_switches() {
//    let args = vec!["-Wl,-a", "-b", "-wl,-c", "-Wl,-d", "-Wl,", "-Wl", "-g"];
//    let result = args.remove_wl_switches();
//    assert!(result.iter().any(|&el| el.starts_with("-Wl,") == false));
//    assert!(result.len() == 6);
//}
