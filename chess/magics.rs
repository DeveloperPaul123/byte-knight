/*
 * magics.rs
 * Part of the byte-knight project
 * Created Date: Friday, August 30th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Fri Oct 11 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{bitboard::Bitboard, definitions::NumberOf};

/// Magic numbers for the bishop piece.
/// Do not modify this array. See the src/bin/generate_magics/main.rs for more information.
pub(crate) const BISHOP_MAGIC_VALUES: [u64; NumberOf::SQUARES] = [
    595196782661861409,
    9011599751790656,
    1157587868464711938,
    5368889448497020992,
    363104992665796616,
    2351513561672975360,
    563843381067792,
    4756082964981432320,
    10448368747282481664,
    576463331465101376,
    306281368336416768,
    54329223212498944,
    4647997459728039936,
    558618382336,
    4616475493292843264,
    18298074725157125,
    5802888257844871696,
    298363612769681952,
    36310478171013392,
    38291044476977344,
    565166175502340,
    563087938684928,
    18085463072982018,
    10232257518911098884,
    4521849023385600,
    19157925500178600,
    153265359544132100,
    10134233100126336,
    148763928212545538,
    2882375229806510592,
    20338766094861312,
    4578950550423552,
    9224014168827371588,
    9531323811908294656,
    4647741238224355393,
    2308097010265751680,
    9011635955965954,
    3461019616464220160,
    9291027873563665,
    231490152105216,
    20286024431183392,
    4757492326287803536,
    1153485004862440448,
    866943203419357696,
    4621001085300057153,
    578652371157504,
    9309239879193870592,
    581106261962031400,
    144704680994545808,
    2826861592969220,
    1152927019361894400,
    2305843009761575428,
    11529782462822613504,
    14136804315897069568,
    4918216958505140224,
    1161929811971871008,
    3460036116207779843,
    9009406887921664,
    5070951929679877,
    2251920081421316,
    2305843011629744650,
    72057663311118596,
    9225659040435045376,
    10385584416998064260,
];

/// Magic numbers for the rook piece.
/// Do not modify this array. See the src/bin/generate_magics/main.rs for more information.
pub(crate) const ROOK_MAGIC_VALUES: [u64; NumberOf::SQUARES] = [
    9259401250783365248,
    306247270842507266,
    612498414153760900,
    72066407864471552,
    2666135411809718528,
    936751505698988288,
    108087490602074496,
    144115471545827588,
    1734308207515435008,
    1180013472457433088,
    563027271239712,
    2814895796535441,
    793337256227243008,
    2599139943536660484,
    562968224367104,
    281507759412480,
    3459364297421897728,
    4611722303384912000,
    5188288058511855617,
    9241424918539667712,
    5497692360960,
    2342013093543936512,
    293881866758422792,
    13917250947790176324,
    13889453096679189632,
    4688317585133996160,
    1729453729462099969,
    9018202962149408,
    6919783027552026689,
    144119588269981824,
    10137501519839744,
    469641138228183296,
    11892387047538817,
    141012374659072,
    563088499675264,
    8798248898560,
    4644354303985665,
    563019310433304,
    2449960675552792728,
    17937964466436,
    432627588964352040,
    9304437114420740096,
    4539342349533204,
    148636401366269960,
    9011597435502720,
    9259963852630654984,
    4634487780893524016,
    9259401385038970897,
    36310549021073664,
    725149978007208064,
    9367768769702658304,
    2305869397761229184,
    7206040947623200000,
    18577417333637632,
    1232015990275965184,
    13344535512576,
    2814840009687106,
    2450239809982054401,
    1196818944233986,
    2832346265172001,
    288793363149557762,
    9259963788190288002,
    5770799976825489410,
    864972604513985025,
];

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MagicNumber {
    pub relevant_bits_mask: u64,
    pub shift: u8,
    pub offset: u64,
    pub magic_value: u64,
}

impl MagicNumber {
    pub fn new(relevant_bits_mask: Bitboard, shift: u8, offset: u64, magic_value: u64) -> Self {
        MagicNumber {
            relevant_bits_mask: relevant_bits_mask.as_number(),
            shift,
            offset,
            magic_value,
        }
    }
    pub fn default() -> Self {
        MagicNumber {
            relevant_bits_mask: 0,
            shift: 0,
            offset: 0,
            magic_value: 0,
        }
    }

    /// Returns the index of the magic number in the table.
    /// This is basically the same formula used to calculate magic numbers, but it's just missing the magic value.
    /// We take into account the shift and offset to calculate the index without the magic value.
    pub fn index(&self, occupancy: Bitboard) -> usize {
        let blockers = occupancy & self.relevant_bits_mask;
        // need to shift
        let blocker_num = blockers.as_number();
        let hash = blocker_num.wrapping_mul(self.magic_value);
        return ((hash >> self.shift) + self.offset) as usize;
    }
}

impl Display for MagicNumber {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "bb {:24} shift {:4} offset {:6} magic {:24}",
            self.relevant_bits_mask, self.shift, self.offset, self.magic_value
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{definitions::Squares, move_generation::MoveGenerator};

    use super::*;

    #[test]
    fn test_magic_number_index() {
        // test a1 for the rook
        let relevant_bits = MoveGenerator::relevant_rook_bits(Squares::A1);
        let blockers = MoveGenerator::create_blocker_permutations(relevant_bits);
        let magic_value = 684547693657194778;
        let magic = MagicNumber::new(
            relevant_bits,
            (64 - relevant_bits.as_number().count_ones()) as u8,
            0,
            magic_value,
        );
        let blocker = blockers[0];
        assert_eq!(magic.index(blocker), 0);

        let mut indexes = Vec::with_capacity(blockers.len());
        for blocker in blockers {
            let index = magic.index(blocker);
            indexes.push(index);
        }
    }
}
