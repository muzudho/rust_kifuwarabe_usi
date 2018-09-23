/// ```
/// ### 以下のコマンドで実行。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_usi
/// cargo run --example main
/// ```

extern crate kifuwarabe_usi;
use kifuwarabe_usi::*;

fn main() {

    let mut starts = 0;
    let line = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL";
    let len = line.chars().count();

    parse_board(&line, &mut starts, len);

}
