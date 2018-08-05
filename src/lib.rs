/// コンピューター将棋 通信部 USIプロトコル Rustフレームワーク
use std::fmt;

/// Fileは筋、Rankは段。1～9を使用。
pub const FILE0: i8 = 0; // 0筋は投了フラグとしても使用。
pub const FILE9: i8 = 9;
pub const FILE10: i8 = 10;
pub const RANK0: i8 = 0;
pub const RANK1: i8 = 1;
pub const RANK10: i8 = 10;
/// 1～9段を a～i段に変換。
pub fn num_to_lower_case(num:i8)->&'static str{
    match num{
        1 =>{"a"},
        2 =>{"b"},
        3 =>{"c"},
        4 =>{"d"},
        5 =>{"e"},
        6 =>{"f"},
        7 =>{"g"},
        8 =>{"h"},
        9 =>{"i"},
        _ =>{panic!("[{}] to lower case.", num)},
    }
}
pub fn file_rank_to_cell(file:i8, rank:i8)->usize{
    debug_assert!(
            (FILE0<file && file<FILE10)
         && (RANK0<rank && rank<RANK10)
         , "(204)file_rank_to_cell file={},rank={}",file, rank);

    (file*10 + rank) as usize
}
pub const STARTPOS_LN: usize = 57;
pub const STARTPOS: &'static str = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL";

/// 駒種類。先後なしの駒と空白。
#[derive(Copy, Clone)]
pub enum PieceType{
    // 玉(King)。
    K,
    // 飛車(Rook)。
    R,
    // 角(Bishop)。
    B,
    // 金(Gold)。
    G,
    // 銀(Silver)。
    S,
    // 桂(kNight)。
    N,
    // 香(Lance)。
    L,
    // 歩(Pawn)。
    P,
    // 竜(Promoted Rook)。
    PR,
    // 馬(Promoted Bishop)。
    PB,
    // 全(Promoted Silver)。
    PS,
    // 圭(Promoted kNight)。
    PN,
    // 杏(Promoted Lance)。
    PL,
    // と(Promoted Pawn)。
    PP,
    // 空マス。
    Space,
    // 要素数より1小さい数。エラー値用に使っても可。
    Num
}
impl fmt::Display for PieceType{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use PieceType::*;
        match *self{
            K => { write!(f,"玉")},
            R => { write!(f,"飛")},
            B => { write!(f,"角")},
            G => { write!(f,"金")},
            S => { write!(f,"銀")},
            N => { write!(f,"桂")},
            L => { write!(f,"香")},
            P => { write!(f,"歩")},
            PB => { write!(f,"竜")},
            PR => { write!(f,"馬")},
            PS => { write!(f,"全")},
            PN => { write!(f,"圭")},
            PL => { write!(f,"杏")},
            PP => { write!(f,"と")},
            Space => { write!(f,"　")},
            Num => { write!(f,"×")},
        }
    }
}

/// 先後付きの駒と空白
#[derive(Copy, Clone)]
pub enum Piece{
    // ▼玉(King)。
    K0,
    // ▼飛車(Rook)。
    R0,
    // ▼角(Bishop)。
    B0,
    // ▼金(Gold)。
    G0,
    // ▼銀(Silver)。
    S0,
    // ▼桂(kNight)。
    N0,
    // ▼香(Lance)。
    L0,
    // ▼歩(Pawn)。
    P0,
    // ▼竜(Promoted Rook)。
    PR0,
    // ▼馬(Promoted Bishop)。
    PB0,
    // ▼全(Promoted Silver)。
    PS0,
    // ▼圭(Promoted kNight)。
    PN0,
    // ▼杏(Promoted Lance)。
    PL0,
    // ▼と(Promoted Pawn)。
    PP0,
    // △玉(King)。
    K1,
    // △飛車(Rook)。
    R1,
    // △角(Bishop)。
    B1,
    // △金(Gold)。
    G1,
    // △銀(Silver)。
    S1,
    // △桂(kNight)。
    N1,
    // △香(Lance)。
    L1,
    // △歩(Pawn)。
    P1,
    // △竜(Promoted Rook)。
    PR1,
    // △馬(Promoted Bishop)。
    PB1,
    // △全(Promoted Silver)。
    PS1,
    // △圭(Promoted kNight)。
    PN1,
    // △杏(Promoted Lance)。
    PL1,
    // △と(Promoted Pawn)。
    PP1,
    // 空マス
    Space,
    // 要素数より1小さい数。該当なしや、エラー値用としても兼用する
    Num
}
impl fmt::Display for Piece{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use Piece::*;
        match *self{
            K0 => { write!(f,"▼玉")},
            R0 => { write!(f,"▼飛")},
            B0 => { write!(f,"▼角")},
            G0 => { write!(f,"▼金")},
            S0 => { write!(f,"▼銀")},
            N0 => { write!(f,"▼桂")},
            L0 => { write!(f,"▼香")},
            P0 => { write!(f,"▼歩")},
            PR0 => { write!(f,"▼竜")},
            PB0 => { write!(f,"▼馬")},
            PS0 => { write!(f,"▼全")},
            PN0 => { write!(f,"▼圭")},
            PL0 => { write!(f,"▼杏")},
            PP0 => { write!(f,"▼と")},
            K1 => { write!(f,"△玉")},
            R1 => { write!(f,"△飛")},
            B1 => { write!(f,"△角")},
            G1 => { write!(f,"△金")},
            S1 => { write!(f,"△銀")},
            N1 => { write!(f,"△桂")},
            L1 => { write!(f,"△香")},
            P1 => { write!(f,"△歩")},
            PR1 => { write!(f,"△竜")},
            PB1 => { write!(f,"△馬")},
            PS1 => { write!(f,"△全")},
            PN1 => { write!(f,"△圭")},
            PL1 => { write!(f,"△杏")},
            PP1 => { write!(f,"△と")},
            Space => { write!(f,"　　")},
            Num => { write!(f,"××")},
        }
    }
}
/// 持ち駒
pub const HAND_PIECE_ARRAY_LN : usize = 14;
pub const HAND_PIECE_ARRAY : [Piece; HAND_PIECE_ARRAY_LN] = [
    Piece::R0,// ▲飛
    Piece::B0,// ▲角
    Piece::G0,// ▲金
    Piece::S0,// ▲銀
    Piece::N0,// ▲桂
    Piece::L0,// ▲香
    Piece::P0,// ▲歩
    Piece::R1,// ▽飛
    Piece::B1,// ▽角
    Piece::G1,// ▽金
    Piece::S1,// ▽銀
    Piece::N1,// ▽桂
    Piece::L1,// ▽香
    Piece::P1// ▽歩
];
pub fn hand_piece_to_num(pc: Piece) -> usize {
    use Piece::*;
    match pc {
        R0 => return 0,
        B0 => return 1,
        G0 => return 2,
        S0 => return 3,
        N0 => return 4,
        L0 => return 5,
        P0 => return 6,
        R1 => return 7,
        B1 => return 8,
        G1 => return 9,
        S1 => return 10,
        N1 => return 11,
        L1 => return 12,
        P1 => return 13,
        _ => panic!("{} is not hand piece.", pc)
    }
}


/// 指し手。4～5桁の文字列に詰め込まれている項目。
///
/// # Members.
/// 
/// * `source_file` - 移動元の筋。
/// * `source_rank` - 移動元の段。
/// * `drop` - 打の場合、打った駒種類。
/// * `destination_file` - 移動先の筋。
/// * `destination_rank` - 移動先の段。
/// * `promotion` - 移動後に成るなら真。
#[derive(Copy,Clone)]
pub struct UsiMovement{
    pub source_file : i8,
    pub source_rank : i8,
    pub drop : PieceType,
    pub destination_file : i8,
    pub destination_rank : i8,
    pub promotion : bool,
}
impl UsiMovement{
    pub fn new()->UsiMovement{
        UsiMovement{
            source_file : 0,
            source_rank : 0,
            drop : PieceType::Space,
            destination_file : 0,
            destination_rank : 0,
            promotion : false,
        }
    }
}
impl fmt::Display for UsiMovement{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {

        // 0筋に移動したら 投了する。
        if self.destination_file==FILE0 { return write!(f,"resign"); }

        match self.drop {
             PieceType::Space => {
                 // 打でないなら
                write!(f, "{}{}{}{}{}",
                    self.source_file,
                    num_to_lower_case(self.source_rank),
                    self.destination_file,
                    num_to_lower_case(self.destination_rank),
                    if self.promotion {"+"}else{""}
                )
             },
             _ => {
                // 打なら
                use PieceType::*;
                write!(f, "{}*{}{}{}",
                    match self.drop {
                        R => { "R" },
                        B => { "B" },
                        G => { "G" },
                        S => { "S" },
                        N => { "N" },
                        L => { "L" },
                        P => { "P" },
                        _  => { panic!("Drop: {}", self.drop); },
                    },
                    self.destination_file,
                    num_to_lower_case(self.destination_rank),
                    if self.promotion {"+"}else{""}
                )
            }
        }
    }
}
impl fmt::Debug for UsiMovement{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Movement(src:{} {}, dst:{} {}, pro:{}, drop:{})",
            self.source_file,
            self.source_rank,
            self.destination_file,
            self.destination_rank,
            self.promotion, self.drop)
    }
}


/// 開始地点から文字列が一致すれば、カーソルを進めて真を返す。
pub fn starts_with_and_forward(
    line: &String,
    starts: &mut usize,
    keyword: &str
) -> bool {
    // ラインの長さ
    let line_len = line.chars().count();
    // キーワードの長さ
    let keyword_len = keyword.chars().count();

    if keyword_len<(line_len-*starts+1) && &line[*starts..(*starts+keyword_len)]==keyword {
        *starts += keyword_len;
        return true;
    }
    return false;
}


/// position コマンドの盤上部分のみ 字句解析。
pub fn parse_banjo(line:&String, starts:&mut usize, len:usize) -> [Piece;100] {

    use Piece::Space;
    // 初期局面の盤面
    let mut ban = [
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
        Space,Space,Space,Space,Space,Space,Space,Space,Space,Space,
    ];

    // 盤部
    use Piece;
    let mut file = FILE9;//９筋から右方向へ読取
    let mut rank = RANK1;
    'ban: while 0<(len-*starts) {
        match &line[*starts..(*starts+1)]{
            "/" => { *starts+=1; file=FILE9; rank+=1; },
            "1" => { *starts+=1;
                ban[file_rank_to_cell(file,rank)] = Space; file-=1;
            },
            "2" => { *starts+=1;
                ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                ban[file_rank_to_cell(file,rank)] = Space; file-=1;
            },
            "3" => { *starts+=1;
                ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                ban[file_rank_to_cell(file,rank)] = Space; file-=1;
            },
            "4" => { *starts+=1;
                for _i_kara in 0..4{
                    ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                }
            },
            "5" => { *starts+=1;
                for _i_kara in 0..5{
                    ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                }
            },
            "6" => { *starts+=1;
                for _i_kara in 0..6{
                    ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                }
            },
            "7" => { *starts+=1;
                for _i_kara in 0..7{
                    ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                }
            },
            "8" => { *starts+=1;
                for _i_kara in 0..8{
                    ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                }
            },
            "9" => { *starts+=1;
                for _i_kara in 0..9{
                    ban[file_rank_to_cell(file,rank)] = Space; file-=1;
                }
            },
            "K" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::K0; file-=1; },
            "R" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::R0; file-=1; },
            "B" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::B0; file-=1; },
            "G" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::G0; file-=1; },
            "S" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::S0; file-=1; },
            "N" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::N0; file-=1; },
            "L" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::L0; file-=1; },
            "P" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::P0; file-=1; },
            "k" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::K1; file-=1; },
            "r" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::R1; file-=1; },
            "b" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::B1; file-=1; },
            "g" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::G1; file-=1; },
            "s" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::S1; file-=1; },
            "n" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::N1; file-=1; },
            "l" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::L1; file-=1; },
            "p" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::P1; file-=1; },
            "+" => {
                *starts+=1;
                match &line[*starts..(*starts+1)]{
                    "R" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PR0; file-=1; },
                    "B" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PB0; file-=1; },
                    "S" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PS0; file-=1; },
                    "N" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PN0; file-=1; },
                    "L" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PL0; file-=1; },
                    "P" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PP0; file-=1; },
                    "r" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PR1; file-=1; },
                    "b" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PB1; file-=1; },
                    "s" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PS1; file-=1; },
                    "n" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PN1; file-=1; },
                    "l" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PL1; file-=1; },
                    "p" => { *starts+=1;  ban[file_rank_to_cell(file,rank)] = Piece::PP1; file-=1; },
                    _ => { panic!(format!("盤部(0) '{}' だった。", &line[*starts..(*starts+1)]));},
                }                    
            },
            _ => {break 'ban;}, // 盤部正常終了
        }
    }

    // 盤面を返却
    ban
}

/// 持ち駒を字句解析
pub fn parse_hand_piece(line:&String, starts:&mut usize, len:usize) -> [i8; HAND_PIECE_ARRAY_LN]{
    // 持ち駒数。増減させたいので、u8 ではなく i8。
    let mut hand_count_arr = [0i8; HAND_PIECE_ARRAY_LN];

    // 持ち駒の読取
    if starts_with_and_forward(line, starts, "-") {
        // 持ち駒なし。        
    } else {
        'mg:loop{
            if 0<(len-*starts){
                // 持ち駒の枚数。
                let mut count = 1;
                match &line[*starts..(*starts+1)]{
                    "1"=>{
                        // 1枚のときは数字は付かないので、10～18 と確定☆
                        match &line[*starts..(*starts+1)]{
                            "0"=>{count=10; *starts+=2;},
                            "1"=>{count=11; *starts+=2;},
                            "2"=>{count=12; *starts+=2;},
                            "3"=>{count=13; *starts+=2;},
                            "4"=>{count=14; *starts+=2;},
                            "5"=>{count=15; *starts+=2;},
                            "6"=>{count=16; *starts+=2;},
                            "7"=>{count=17; *starts+=2;},
                            "8"=>{count=18; *starts+=2;},
                            _ => { panic!("[{}] is not hand piece.", &line[*starts..(*starts+2)]);},
                        }
                    },
                    "2"=>{count=2; *starts+=1;},
                    "3"=>{count=3; *starts+=1;},
                    "4"=>{count=4; *starts+=1;},
                    "5"=>{count=5; *starts+=1;},
                    "6"=>{count=6; *starts+=1;},
                    "7"=>{count=7; *starts+=1;},
                    "8"=>{count=8; *starts+=1;},
                    "9"=>{count=9; *starts+=1;},
                    _ => {},// 駒の名前なら次へ (エラーの場合は取りこぼす)
                }

                use Piece::*;
                let piece : Piece;
                match &line[*starts..(*starts+1)]{
                    "R"=>{ piece=R0; *starts+=1; },
                    "B"=>{ piece=B0; *starts+=1; },
                    "G"=>{ piece=G0; *starts+=1; },
                    "S"=>{ piece=S0; *starts+=1; },
                    "N"=>{ piece=N0; *starts+=1; },
                    "L"=>{ piece=L0; *starts+=1; },
                    "P"=>{ piece=P0; *starts+=1; },
                    "r"=>{ piece=R1; *starts+=1; },
                    "b"=>{ piece=B1; *starts+=1; },
                    "g"=>{ piece=G1; *starts+=1; },
                    "s"=>{ piece=S1; *starts+=1; },
                    "n"=>{ piece=N1; *starts+=1; },
                    "l"=>{ piece=L1; *starts+=1; },
                    "p"=>{ piece=P1; *starts+=1; },
                    _ => { break 'mg; }, // 持駒部 正常終了
                }

                hand_count_arr[hand_piece_to_num(piece)] = count;
            }//if
        }//loop
    }//else

    return hand_count_arr;
}

/// 指し手文字列から、打った駒種類を抽出します。
///
/// # Examples input.
/// 
/// * `7g7f`
/// * `B*5e`
/// * `3d3c+`
/// 
/// # Examples return.
/// 
/// * (true, ...) - Successful.
/// * (false, ...) - End parse.
pub fn parse_movement(
    line: &String,
    starts: &mut usize,
    len: usize
) -> (bool, UsiMovement) {

    let mut result = UsiMovement{
        source_file : -1,
        source_rank : -1,
        drop : PieceType::Space,
        destination_file : -1,
        destination_rank : -1,
        promotion : false,
    };

    // 4文字か5文字あるはず。
    if (len-*starts)<4{
        // 指し手読取終了時にここを通るぜ☆（＾～＾）
        // 残り４文字もない。
        return (false, result);
    }

    // 1文字目と2文字目
    match &line[*starts..(*starts+1)]{
        // 1文字目が駒だったら打。2文字目は必ず「*」なはずなので読み飛ばす。
        "R" => { *starts+= 2; result.drop= PieceType::R },
        "B" => { *starts+= 2; result.drop= PieceType::B },
        "G" => { *starts+= 2; result.drop= PieceType::G },
        "S" => { *starts+= 2; result.drop= PieceType::S },
        "N" => { *starts+= 2; result.drop= PieceType::N },
        "L" => { *starts+= 2; result.drop= PieceType::L },
        "P" => { *starts+= 2; result.drop= PieceType::P },
        _ => {
            // 残りは「筋の数字」、「段のアルファベット」のはず。
            result.source_file = match &line[*starts..(*starts+1)]{
                "1" => 1,
                "2" => 2,
                "3" => 3,
                "4" => 4,
                "5" => 5,
                "6" => 6,
                "7" => 7,
                "8" => 8,
                "9" => 9,
                _ => {panic!(format!("(1) '{}' だった。", &line[*starts..(*starts+1)]));},
            };
            *starts+=1;

            result.source_rank = match &line[*starts..(*starts+1)]{
                "a" => 1,
                "b" => 2,
                "c" => 3,
                "d" => 4,
                "e" => 5,
                "f" => 6,
                "g" => 7,
                "h" => 8,
                "i" => 9,
                _ => {panic!(format!("(2) '{}' だった。", &line[*starts..(*starts+1)]));},
            };
            *starts+=1;
        },
    }

    // 3文字目
    result.destination_file = match &line[*starts..(*starts+1)]{
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => {panic!(format!("(3) '{}' だった。", &line[*starts..(*starts+1)]));},
    };
    *starts+=1;
    
    // 4文字目
    result.destination_rank = match &line[*starts..(*starts+1)]{
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        "i" => 9,
        _ => {panic!(format!("(4) '{}' だった。", &line[*starts..(*starts+1)]));},
    };
    *starts+=1;
    
    // 5文字に「+」があれば成り。
    if starts_with_and_forward(line, starts, "+") {
        result.promotion = true;
    }

    // 続きにスペース「 」が１つあれば読み飛ばす
    if starts_with_and_forward(line, starts, " ") {
    }

    // 残りは「筋の数字」、「段のアルファベット」のはず。成り
    return (true, result);
}

/// position コマンド読取
pub fn parse_position(
    line: &String,
    callback0: fn([i8; HAND_PIECE_ARRAY_LN]),
    callback1: fn([Piece;100]),
    callback2: fn(bool, UsiMovement)
){

    let mut starts = 0;

    // 全体の長さ
    let len = line.chars().count();



    let ban : [Piece;100];
    if starts_with_and_forward(line, &mut starts, "position startpos") {
        // 別途用意した平手初期局面文字列を読取
        let mut local_starts = 0;

        // position コマンド 盤上部分のみ 読取
        ban = parse_banjo(&STARTPOS.to_string(), &mut local_starts, STARTPOS_LN);

        if starts_with_and_forward(line, &mut starts, " ") {
            // 読み飛ばし。
        }
    }else if starts_with_and_forward(line, &mut starts, "position sfen ") {

        // position コマンド 盤上部分のみ 読取
        ban = parse_banjo(&line, &mut starts, len);

        if starts_with_and_forward(line, &mut starts, " ") {
            // 読み飛ばし。
        }

        // 先後も読み飛ばす。
        if starts_with_and_forward(line, &mut starts, "w") {
            // 読み飛ばし。
        }else if starts_with_and_forward(line, &mut starts, "b") {
            // 読み飛ばし。
        }

        if starts_with_and_forward(line, &mut starts, " ") {
            // 読み飛ばし。
        }

        // 持ち駒数。増減させたいので、u8 ではなく i8。
        let hand_count_arr : [i8; HAND_PIECE_ARRAY_LN] = parse_hand_piece(line, &mut starts, len);
        callback0(hand_count_arr);


        if starts_with_and_forward(line, &mut starts, " 1 ") {
            // 読み飛ばし。
        }
    }else{
        panic!("'position startpos' でも、'position sfen ' でも始まらなかった。");
    }

    // 盤を返す。
    callback1(ban);

    if starts_with_and_forward(line, &mut starts, "moves") {
            // 読み飛ばし。
    }

    if starts_with_and_forward(line, &mut starts, " ") {
            // 読み飛ばし。
    }

    // 指し手を1つずつ返すぜ☆（＾～＾）
    loop {
        let (successful, umov) = parse_movement(line, &mut starts, len);
        callback2(successful, umov);
        if !successful {
            // 読取終了時(失敗時)。
            break;
        }
    } // loop
}