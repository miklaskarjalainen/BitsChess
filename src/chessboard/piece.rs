use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PieceType {
    None   = 0,
    Pawn   = 1,
    Knight = 2,
    Bishop = 3,
    Rook   = 4,
    Queen  = 5,
    King   = 6,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PieceColor {
    White = 0,
    Black = 1
}

impl PieceColor {
    #[must_use]
    fn from_u8(val: u8) -> PieceColor {
        unsafe { 
            std::mem::transmute(val & 0b1)
        }
    }

    #[must_use]
    pub fn flipped(&self) -> PieceColor {
        Self::from_u8((*self as u8) + 1)
    }

    pub fn flip(&mut self) {
        *self = self.flipped();
    }
}

impl PieceType {
    const PIECE_VALUE:[i32; 7] = [0, 100, 300, 320, 500, 900, 0];

    pub const fn get_value(&self) -> i32 {
        return PieceType::PIECE_VALUE[*self as usize];
    }

    fn from_char(ch: char) -> PieceType {
        match ch.to_ascii_lowercase() {
            'p' => { PieceType::Pawn }
            'n' => { PieceType::Knight }
            'b' => { PieceType::Bishop }
            'r' => { PieceType::Rook }
            'q' => { PieceType::Queen }
            'k' => { PieceType::King }
            _ => { PieceType::None }
        }
    }

    fn to_char(&self) -> char {
        match self {
            PieceType::Pawn => { 'p' }
            PieceType::Knight => { 'n' }
            PieceType::Bishop => { 'b' }
            PieceType::Rook => { 'r' }
            PieceType::Queen => { 'q' }
            PieceType::King => { 'k' }
            _ => { ' ' }
        }
    }

    fn from_u8(val: u8) -> PieceType {
        unsafe { 
            std::mem::transmute(val & 0b111)
        }
    }
}

// Bit 7 -- Color of the piece (0 is white, 1 is black)
// Bit 6-5 are unused
// Bit 4 -- Castle flag for Kings only (0 can castle)
// Bit 3 -- Piece has moved flag (1 has moved)
// Bits 2-0 Piece Type
//  0 -- Empty Square
//  1 -- Pawn
//  2 -- Knight
//  3 -- Bishop
//  4 -- Rook
//  5 -- Queen
//  6 -- King
//  7 -- Not used
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece(pub u8);

impl Piece {
    pub const fn new(flags: u8) -> Self {
        Self(flags)
    }

    pub fn get_piece_value(&self) -> i32 {
        self.get_piece_type().get_value()
    }

    // 0 = white pawn, 1 = white knight ... 6 = black pawn
    pub fn get_piece_index(&self) -> usize {
        let t = self.get_piece_type();
        let c = self.get_color();

        assert!(t != PieceType::None);
        
        let mut index = (t as u8) - 1;
        if c == PieceColor::Black {
            index += 6
        }
        index as usize
    }

    pub fn get_color(&self) -> PieceColor {
        PieceColor::from_u8(self.0 >> 7)
    }

    pub fn is_white(&self) -> bool {
        ((self.0 >> 7) & 0b1) == 0
    }

    pub fn is_black(&self) -> bool {
        ((self.0 >> 7) & 0b1) == 1
    }

    pub fn has_moved(&self) -> bool {
        ((self.0 >> 3) & 0b1) == 1
    }

    pub fn set_moved(&mut self, moved: bool) {
        if moved {
            self.0 |= 0b1 << 3;
        }
        else {
            self.0 &= 0b11110111;
        }
    }

    pub fn can_castle(&self) -> bool {
        ((self.0 >> 4) & 0b1) == 0
    }

    pub fn set_can_castle(&mut self, flag: bool) {
        if flag {
            self.0 &= 0b11101111;
        }
        else {
            self.0 |= 0b1 << 4;
        }
    }

    pub fn is_none(&self) -> bool {
        (self.0 & 0b111) == 0 || self.0 == 0xFF
    }

    pub fn set_piece(&mut self, t: PieceType) {
        self.0 &= 0b11111000; // unset piece bits
        self.0 |= (t as u8) & 0b111;
    }

    pub fn get_piece(&self) -> u8 {
        self.0 & 0b111
    }

    pub fn get_piece_type(&self) -> PieceType {
        PieceType::from_u8(self.get_piece())
    }

    pub fn to_char(&self) -> char {
        let c = self.get_piece_type().to_char();
        if c != ' ' && self.is_white() {
            return c.to_ascii_uppercase();
        }
        c
    }

    pub fn from_char(ch: char) -> Piece {
        let mut piece = Piece::new(0);
        piece.0 |= (PieceType::from_char(ch) as u8) & 0b111; // Piece Type
        if !ch.is_uppercase() {
            piece.0 |= 0b1 << 7;
        }
        
        piece
    }

}

impl fmt::Display for Piece {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.pad(format!("{{piece: {:?}, color: {:?}}}", self.get_piece_type(), self.get_color()).as_str())
    }
}
