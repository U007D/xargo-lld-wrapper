#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![deny(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts,
unsafe_code, unused_import_braces, unused_qualifications)]
#![allow(unused_variables)]
#![feature(inclusive_range_syntax)]
#![feature(associated_consts)]

use std::path::Path;

trait ArgsExtensionMethods<'a> where Self: IntoIterator {
//    const NO_START_FILES_SWITCH: &'static str = "-nostartfiles";
//    const NO_DEFAULT_LIBS_SWITCH: &'static str = "-nodefaultlibs";
//    const WL_SWITCH_PREFIX: &'static str = "-Wl,";

    fn find_arg_after_switch(self, switch: &str) -> Option<Self::Item>;

    //    fn remove_first_arg(self) -> Self;
//    fn remove_output_filename_switches_and_params(self) -> Self;
//    fn remove_nostartfiles_nodefaultlibs_switches(self) -> Self;
//    fn remove_wl_switches(self) -> Self;
}

impl<'a, T> ArgsExtensionMethods<'a> for T where T: IntoIterator<Item = &'a &'a str> {
    //TODO: If multiple -o params specified, return (last occurrence of) OUTPUT_FILENAME parameter, if present
    fn find_arg_after_switch(self, switch_name: &str) -> Option<Self::Item> {
        self.into_iter()
            .skip_while(|el| **el != switch_name)
            .skip(1) //move past -o switch
            .next()
    }
}
//    fn remove_first_arg(self) -> Vec<&'a str> {
//        self.into_iter()
//            .skip(1)
//            .take_while(|_| true)
//            .collect::<Vec<_>>()
//    }
//
//    //Delete all of both the OUTPUT_SWITCH and the immediately following argument (OUTPUT_FILENAME) if one is present
//    fn remove_output_filename_switches_and_params(self) -> Self {
//        self.into_iter()
//            .scan(false, |found_switch, el| match *found_switch {
//                true => { *found_switch = false; Some(None) },
//                false => match el == Self::OUTPUT_SWITCH {
//                    true => { *found_switch = true; Some(None) },
//                    false => Some(Some(el)) }})
//            .filter(|el| el.is_some())
//            .map(|el| el.expect("Unwrapped element found in list of Option<&str>!"))
//            .collect::<Vec<_>>()
//    }
//
//    fn remove_nostartfiles_nodefaultlibs_switches(self) -> Self {
//        self.into_iter()
//            .filter(|&el| el != Self::NO_START_FILES_SWITCH && el != Self::NO_DEFAULT_LIBS_SWITCH)
//            .collect::<Vec<_>>()
//    }
//
//    fn remove_wl_switches(self) -> Self {
//        self.into_iter()
//            .map(|el| match el.starts_with(Self::WL_SWITCH_PREFIX) {
//                true => &el[Self::WL_SWITCH_PREFIX.as_bytes().len()..],
//                false => &el, })
//            .filter(|el| !el.trim().is_empty())
//            .collect::<Vec<_>>()
//    }
//}
//
fn get_linker_uri() -> String
{
    const LLD_PATH_KEY: &'static str = "LLD_PATH";
    const LLD_FILENAME_KEY: &'static str = "LLD_FILENAME";

    let path = match std::env::var(LLD_PATH_KEY) { Ok(v) => v, Err(_) => "".to_string() };
    let filename = match std::env::var(LLD_FILENAME_KEY) { Ok(v) => v, Err(_) => "ld.lld".to_string() };
    std::path::Path::new(&path).join(&filename).to_string_lossy().into_owned()
}

//TODO: Auto-convert from Vec<String> to Vec<&str>; https://is.gd/UbOlU2
//(implement std::convert::From<Vec<String>> for Vec<&str>)
pub fn lib_main<'a, T>(args: T) where T: IntoIterator<Item=&'a str>
{
    const OUTPUT_SWITCH_NAME: &'static str = "-o";
    let fake_args = vec!["a", "b", "-o", "d"];

    let target = Path::new(fake_args.find_arg_after_switch(OUTPUT_SWITCH_NAME).unwrap());
//    let fixed_args = args.remove_first_arg()
//                        .remove_nostartfiles_nodefaultlibs_switches()
//                        .remove_wl_switches();
//
//    std::process::Command::new(get_linker_uri()).args(fixed_args).spawn().expect("lld command failed");
//    std::process::Command::new("cp").args(&[target.to_str().unwrap(),
//                                          &("./target/".to_string() + target.file_name().unwrap().to_str().unwrap())]);
    println!("{:?}{:?}", target, Path::new(fake_args.find_arg_after_switch(OUTPUT_SWITCH_NAME).unwrap()));
}

#[cfg(test)]
mod tests;
