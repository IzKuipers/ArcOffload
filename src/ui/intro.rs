use colored::Colorize;

use crate::env::VERSION;

pub fn display_intro() {
    let art = vec![
        "    _           ___   __  __ _              _ ",
        "   /_\\  _ _ __ / _ \\ / _|/ _| |___  __ _ __| |",
        "  / _ \\| '_/ _| (_) |  _|  _| / _ \\/ _` / _` |",
        " /_/ \\_\\_| \\__|\\___/|_| |_| |_\\___/\\__,_\\__,_|",
    ];

    for i in 0..4 {
        println!("{}", art[i].blue());
    }

    println!(
        "\nArcOffload {}. Created by {} for the {}.\n",
        VERSION.yellow(),
        "Izaak Kuipers".purple().bold(),
        "ArcOS Project".blue()
    )
}
