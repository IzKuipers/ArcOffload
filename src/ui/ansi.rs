use super::error::error_and_exit;

pub fn try_ansi_or_exit() -> () {
    match enable_ansi_support::enable_ansi_support() {
        Ok(()) => (),
        Err(_) => {
            error_and_exit("Could not enable ANSI support!", 1);

            return ();
        }
    }
}
