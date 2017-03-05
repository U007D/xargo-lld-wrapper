use super::*;

#[test]
fn app_call_with_no_params_succeeds() {
    lib_main(Vec::<&str>::new());
}

#[test]
fn get_output_filename_extracts_output_filename() {
    let args = ["-o", "testOutputFilename"].to_vec();
    let result = args.get_output_filename();
    assert!(result == Some("testOutputFilename"));
}

#[test]
fn get_output_filename_returns_none_when_given_insufficient_output_args() {
    let args = vec!["-o"];
    let result = args.get_output_filename();
    assert!(result == None);
}

#[test]
fn get_output_filename_returns_none_when_given_no_output_args() {
    let args = vec!["-anything_but_o"];
    let result = args.get_output_filename();
    assert!(result == None);
}

#[test]
fn remove_output_filename_switch_and_param() {
    let args = vec!["-a", "-o", "remove_me", "-z"];
    let result = args.remove_output_filename_switches_and_params();
    assert!(!result.iter().any(|&el| el == "-o" || el == "remove_me"));
    assert!(result.len() == 2);
}

#[test]
fn remove_output_filename_switch_and_dash_o_param() {
    let args = vec!["-a", "-o", "-o", "-z"];
    let result = args.remove_output_filename_switches_and_params();
    assert!(!result.iter().any(|&el| el == "-o" || el == "remove_me"));
    assert!(result.len() == 2);
}

#[test]
fn remove_output_filename_switch_and_two_dash_o_params() {
    let args = vec!["-a", "-o", "remove_me", "-o", "-z"];
    let result = args.remove_output_filename_switches_and_params();
    assert!(!result.iter().any(|&el| el == "-o" || el == "remove_me"));
    assert!(result.len() == 1);
}

#[test]
fn remove_output_filename_switch_no_param() {
    let args = vec!["-a", "-o"];
    let result = args.remove_output_filename_switches_and_params();
    assert!(!result.iter().any(|&el| el == "-o" || el == "remove_me"));
    assert!(result.len() == 1);
}

#[test]
fn remove_nostatfiles_switch() {
    let args = vec!["-a", "-nostartfiles", "-nostartfiles", "-y", "-z"];
    let result = args.remove_nostartfiles_switches();
    assert!(!result.iter().any(|&el| el == "-nostartfiles"));
    assert!(result.len() == 3);
}

#[test]
fn remove_wl_switches() {
    let args = vec!["-Wl,-a", "-b", "-wl,-c", "-Wl,-d", "-Wl,", "-Wl", "-g"];
    let result = args.remove_wl_switches();
    assert!(result.iter().any(|&el| el.starts_with("-Wl,") == false));
    assert!(result.len() == 6);
}
