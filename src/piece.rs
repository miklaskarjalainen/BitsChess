/// # Type  
/// Represents the type of a chess piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// # Color  
/// Used to represent color of pieces and track the current turn.
/// * [PieceColor::White] = 0
/// * [PieceColor::Black] = 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceColor {
    White = 0,
    Black = 1
}

impl PieceColor {
    /// Creates [PieceColor] enum from a [u8], only uses the first bit of the [u8].
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::PieceColor;
    /// assert_eq!(PieceColor::from_u8(0b0), PieceColor::White);
    /// assert_eq!(PieceColor::from_u8(0b1), PieceColor::Black);
    /// assert_eq!(PieceColor::from_u8(0b1111_1110), PieceColor::White);
    /// assert_eq!(PieceColor::from_u8(0b1111_1111), PieceColor::Black);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn from_u8(val: u8) -> Self {
        unsafe { 
            std::mem::transmute(val & 0b1)
        }
    }

    /// Returns the flipped version of [PieceColor].
    ///   
    /// * [PieceColor::White] -> [PieceColor::Black]    
    /// * [PieceColor::Black] -> [PieceColor::White]
    /// # Examples  
    /// ```rust
    /// use bitschess::PieceColor;
    /// assert_eq!(PieceColor::Black.flipped(), PieceColor::White);
    /// assert_eq!(PieceColor::White.flipped(), PieceColor::Black);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn flipped(self) -> Self {
        unsafe {
            // Works, because PieceColor uses only 1 bit.
            // 0b1^1 -> 0b0, 0b0^1 -> 0b1
            std::mem::transmute(self as u8 ^ 0b1)
        }
    }

    /// Flips [PieceColor].
    ///   
    /// * [PieceColor::White] -> [PieceColor::Black]    
    /// * [PieceColor::Black] -> [PieceColor::White]
    /// # Examples  
    /// ```rust
    /// use bitschess::PieceColor;
    /// let mut col = PieceColor::Black;
    /// col.flip();
    /// assert_eq!(col, PieceColor::White);
    /// col.flip();
    /// assert_eq!(col, PieceColor::Black);
    /// ```
    #[inline(always)]
    pub fn flip(&mut self) {
        *self = self.flipped();
    }
}

impl PieceType {
    /// Get's a "value" of a piece. Mainly used for in chess engines, but those should define their own values.  
    #[must_use]
    #[deprecated]
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn get_value(self) -> i32 {
        const PIECE_VALUE:[i32; 7] = [0, 100, 300, 320, 500, 900, 0];
        PIECE_VALUE[self as usize]
    }

    /// Get's [usize] index for a piece type. Used to index into arrays.
    /// The piece_type cannot be [PieceType::None] as it will result in 0usize-1.
    #[must_use]
    #[inline(always)]
    pub const fn get_index(self) -> usize {
        (self as usize) - 1
    }

    /// Get's [usize] index for a piece type. Used to index into arrays.
    /// The piece_type cannot be [PieceType::None].
    /// 
    /// `0..=5 are used for white pieces.`  
    /// `6..=11 are used for black pieces.`
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::{PieceType, PieceColor};
    /// assert_eq!(PieceType::Pawn.get_side_index(PieceColor::White), 0);
    /// assert_eq!(PieceType::Knight.get_side_index(PieceColor::White), 1);
    /// assert_eq!(PieceType::Queen.get_side_index(PieceColor::White), 4);
    /// assert_eq!(PieceType::King.get_side_index(PieceColor::White), 5);
    /// assert_eq!(PieceType::Pawn.get_side_index(PieceColor::Black), 6);
    /// assert_eq!(PieceType::Knight.get_side_index(PieceColor::Black), 7);
    /// assert_eq!(PieceType::Queen.get_side_index(PieceColor::Black), 10);
    /// assert_eq!(PieceType::King.get_side_index(PieceColor::Black), 11);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn get_side_index(self, side: PieceColor) -> usize {
        (side as usize) * 6 + self.get_index()
    }

    /// Creates a [PieceType] from a char.
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::{PieceType};
    /// 
    /// // Valid inputs
    /// assert_eq!(PieceType::from_char('p'), PieceType::Pawn);
    /// assert_eq!(PieceType::from_char('n'), PieceType::Knight);
    /// assert_eq!(PieceType::from_char('b'), PieceType::Bishop);
    /// assert_eq!(PieceType::from_char('r'), PieceType::Rook);
    /// assert_eq!(PieceType::from_char('q'), PieceType::Queen);
    /// assert_eq!(PieceType::from_char('k'), PieceType::King);
    /// 
    /// // Invalids
    /// assert_eq!(PieceType::from_char('?'), PieceType::None);
    /// assert_eq!(PieceType::from_char('@'), PieceType::None);
    /// assert_eq!(PieceType::from_char('\0'), PieceType::None);
    /// assert_eq!(PieceType::from_char('\n'), PieceType::None);
    /// ```
    #[must_use]
    pub const fn from_char(ch: char) -> Self {
        match ch.to_ascii_lowercase() {
            'p' => { Self::Pawn }
            'n' => { Self::Knight }
            'b' => { Self::Bishop }
            'r' => { Self::Rook }
            'q' => { Self::Queen }
            'k' => { Self::King }
            _ => { Self::None }
        }
    }

    /// Get's corresponding [char] for a piece type.
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::{PieceType};
    /// assert_eq!(PieceType::None.to_char(), ' ');
    /// assert_eq!(PieceType::Pawn.to_char(), 'p');
    /// assert_eq!(PieceType::Knight.to_char(), 'n');
    /// assert_eq!(PieceType::Bishop.to_char(), 'b');
    /// assert_eq!(PieceType::Rook.to_char(), 'r');
    /// assert_eq!(PieceType::Queen.to_char(), 'q');
    /// assert_eq!(PieceType::King.to_char(), 'k');
    /// ```
    #[must_use]
    pub const fn to_char(self) -> char {
        match self {
            Self::Pawn => { 'p' }
            Self::Knight => { 'n' }
            Self::Bishop => { 'b' }
            Self::Rook => { 'r' }
            Self::Queen => { 'q' }
            Self::King => { 'k' }
            Self::None => { ' ' }
        }
    }

    /// Creates [PieceType] enum from a [u8].  
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::PieceType;
    /// assert_eq!(PieceType::from_u8(0), PieceType::None);
    /// assert_eq!(PieceType::from_u8(1), PieceType::Pawn);
    /// assert_eq!(PieceType::from_u8(2), PieceType::Knight);
    /// assert_eq!(PieceType::from_u8(3), PieceType::Bishop);
    /// assert_eq!(PieceType::from_u8(4), PieceType::Rook);
    /// assert_eq!(PieceType::from_u8(5), PieceType::Queen);
    /// assert_eq!(PieceType::from_u8(6), PieceType::King);
    /// ```
    #[inline(always)]
    pub const fn from_u8(val: u8) -> Self {
        unsafe { 
            std::mem::transmute(val & 0b111)
        }
    }
}

/// # A Piece
/// Piece is represented with 8 bits where:  
/// * bits 0-2 are used for [PieceType].
/// * bits 3-6 are unused.
/// * bit 7 is used for color (0 is white, 1 is black)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece(pub u8);

impl Piece {
    #[must_use]
    #[inline(always)]
    pub const fn new(flags: u8) -> Self {
        Self(flags)
    }

    /// Get's [usize] index for a piece. Used to index into arrays.  
    /// Internally calls [PieceType::get_side_index].
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::Piece;
    /// let white_pawn = Piece::from_char('P');
    /// let black_pawn = Piece::from_char('p');
    /// assert_eq!(white_pawn.get_piece_index(), 0);
    /// assert_eq!(black_pawn.get_piece_index(), 6);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn get_piece_index(self) -> usize {
        let t = self.get_piece_type();
        let c = self.get_color();
        t.get_side_index(c)
    }

    /// Returns the [PieceColor] index for a piece.  
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::{Piece, PieceColor};
    /// let white_pawn = Piece::from_char('P');
    /// let black_pawn = Piece::from_char('p');
    /// assert_eq!(white_pawn.get_color(), PieceColor::White);
    /// assert_eq!(black_pawn.get_color(), PieceColor::Black);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn get_color(self) -> PieceColor {
        PieceColor::from_u8(self.0 >> 7)
    }

    /// # Examples  
    /// ```rust
    /// use bitschess::Piece;
    /// let white_pawn = Piece::from_char('P');
    /// let black_pawn = Piece::from_char('p');
    /// assert_eq!(white_pawn.is_white(), true);
    /// assert_eq!(black_pawn.is_white(), false);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn is_white(self) -> bool {
        ((self.0 >> 7) & 0b1) == 0
    }

    /// # Examples  
    /// ```rust
    /// use bitschess::Piece;
    /// let white_pawn = Piece::from_char('P');
    /// let black_pawn = Piece::from_char('p');
    /// assert_eq!(white_pawn.is_black(), false);
    /// assert_eq!(black_pawn.is_black(), true);
    /// ```
    #[must_use]
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn is_black(self) -> bool {
        ((self.0 >> 7) & 0b1) == 1
    }

    /// # Examples  
    /// ```rust
    /// use bitschess::Piece;
    /// let none_piece1 = Piece::new(0);
    /// let none_piece2 = Piece::from_char(' ');
    /// let white_pawn = Piece::from_char('P');
    /// let black_pawn = Piece::from_char('p');
    /// assert_eq!(none_piece1.is_none(), true);
    /// assert_eq!(none_piece2.is_none(), true);
    /// assert_eq!(white_pawn.is_none(), false);
    /// assert_eq!(black_pawn.is_none(), false);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn is_none(self) -> bool {
        (self.0 & 0b111) == 0 || self.0 == 0xFF
    }

    /// Changes the bits which represent the type of the piece to represent a new [PieceType].  
    /// Other bits are unchanged.
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::{Piece, PieceType};
    /// let mut white_pawn = Piece::from_char('P');
    /// let mut black_pawn = Piece::from_char('p');
    /// white_pawn.set_piece(PieceType::Queen);
    /// black_pawn.set_piece(PieceType::King);
    /// 
    /// assert_eq!(white_pawn, Piece::from_char('Q')); // white queen
    /// assert_eq!(black_pawn, Piece::from_char('k')); // black queen
    /// ```
    #[inline(always)]
    pub fn set_piece(&mut self, t: PieceType) {
        self.0 &= 0b11111000; // unset piece bits
        self.0 |= (t as u8) & 0b111;
    }

    /// Gets the underlying bits which represents the [PieceType] of the piece.  
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::{Piece, PieceType};
    /// let mut white_pawn = Piece::from_char('P');
    /// let mut black_pawn = Piece::from_char('p');
    /// let mut black_queen = Piece::from_char('q');
    /// assert_eq!(white_pawn.get_piece(), PieceType::Pawn as u8);
    /// assert_eq!(black_pawn.get_piece(), PieceType::Pawn as u8);
    /// assert_eq!(black_queen.get_piece(), PieceType::Queen as u8);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn get_piece(self) -> u8 {
        self.0 & 0b111
    }
    
    /// Gets the [PieceType] of the piece.  
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::{Piece, PieceType};
    /// let mut white_pawn = Piece::from_char('P');
    /// let mut black_knight = Piece::from_char('n');
    /// let mut black_queen = Piece::from_char('q');
    /// assert_eq!(white_pawn.get_piece_type(), PieceType::Pawn);
    /// assert_eq!(black_knight.get_piece_type(), PieceType::Knight);
    /// assert_eq!(black_queen.get_piece_type(), PieceType::Queen);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn get_piece_type(self) -> PieceType {
        PieceType::from_u8(self.get_piece())
    }

    /// Gets the [char] for the piece.  
    /// For white pieces the char is uppercase, and for black lowercase.  
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::{Piece, PieceType};
    /// let mut none_piece = Piece::from_char(' ');
    /// let mut white_pawn = Piece::from_char('P');
    /// let mut black_knight = Piece::from_char('n');
    /// let mut black_queen = Piece::from_char('q');
    /// assert_eq!(none_piece.to_char(), ' ');
    /// assert_eq!(white_pawn.to_char(), 'P');
    /// assert_eq!(black_knight.to_char(), 'n');
    /// assert_eq!(black_queen.to_char(), 'q');
    /// ```
    #[must_use]
    pub const fn to_char(self) -> char {
        let c = self.get_piece_type().to_char();
        if c != ' ' && self.is_white() {
            return c.to_ascii_uppercase();
        }
        c
    }

    /// Constructs a [Piece] from a [char].  
    /// [PieceColor] is [PieceColor::White] if [char] is uppercase, if lowercase then [PieceColor::Black].
    /// 
    /// # Examples  
    /// ```rust
    /// use bitschess::{Piece, PieceType};
    /// let mut none_piece = Piece::from_char(' ');
    /// let mut black_king = Piece::from_char('k');
    /// let mut white_queen = Piece::from_char('Q');
    /// assert_eq!(none_piece.get_piece_type(), PieceType::None);
    /// 
    /// assert_eq!(black_king.get_piece_type(), PieceType::King);
    /// assert_eq!(black_king.is_black(), true);
    /// 
    /// assert_eq!(white_queen.get_piece_type(), PieceType::Queen);
    /// assert_eq!(white_queen.is_white(), true);
    /// ```
    #[must_use]
    pub const fn from_char(ch: char) -> Self {
        let mut piece = Self::new(0);
        piece.0 |= (PieceType::from_char(ch) as u8) & 0b111; // Piece Type
        if !ch.is_ascii_uppercase() {
            piece.0 |= 0b1 << 7;
        }
        
        piece
    }

}

impl std::fmt::Display for Piece {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.pad(format!("{{piece: {:?}, color: {:?}}}", self.get_piece_type(), self.get_color()).as_str())
    }
}
