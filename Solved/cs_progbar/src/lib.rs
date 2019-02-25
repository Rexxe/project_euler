extern crate indicatif;

use indicatif::{ProgressBar,ProgressStyle};

static mut PB : Option<ProgressBar> = None; // indicatif::

#[no_mangle]
pub extern "C" fn initialize_bar(length : i32) {
    unsafe {
        PB = Some(ProgressBar::new(length as u64));
        let sty = ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}");
        match &PB {
            Some(prog) => prog.set_style(sty),
            None => panic!("Set style failed!"),
        }


    }
}

#[no_mangle]
pub extern "C" fn advance_bar(by_amount : i32) {
    unsafe {
        match &PB {
            Some(prog) => prog.inc(by_amount as u64),
            None => panic!("Must initialize bar first!"),
        }
    }
}

#[no_mangle]
pub extern "C" fn set_bar_message() {
    unsafe {
        match &PB {
            Some(prog) => prog.finish_with_message("Milkshakes!"),
            None => panic!("Must initialize bar first!"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
