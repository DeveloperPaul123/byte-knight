/*
 * magics.rs
 * Part of the byte-knight project
 * Created Date: Friday, August 30th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Sun Sep 01 2024
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
    2378472692963639296,
    1157270352691232,
    4614104948303671428,
    2393087057854784,
    9614129711026176,
    299092932564098,
    9229002120647180320,
    2311754361715359744,
    577309990348455952,
    1154056234983911424,
    2216341668128,
    144282315186503937,
    4616475559965229056,
    585256470532,
    5764627889874141284,
    299564288140525573,
    2324003166095933440,
    281544786837634,
    581615263955296256,
    720575943080935680,
    9322451280234424840,
    17727750160,
    4503896651736320,
    2199602593952,
    4648031749941886978,
    40682073948168,
    72057972297073664,
    1099513987072,
    329396641386725376,
    576465150920377648,
    290482451925567618,
    904097169664,
    2459001169361502721,
    288231554063929360,
    1301685497638223872,
    1408753532928,
    35184640593920,
    145276830717413376,
    9241388711797604624,
    1315051649740341282,
    1161937535459396160,
    26401298272264,
    576742365457940490,
    9225624970951131136,
    1153351423325454336,
    5206302563895418882,
    4629709526631710922,
    2324816036167680,
    9223376454782944320,
    9225626045390135296,
    2306124486539304976,
    288230685408309824,
    1162080437003288868,
    9007336861503492,
    9295464850195505408,
    288520681636315168,
    2305844110067769352,
    15132094765753575170,
    5188719066569376256,
    5765242217866338564,
    2815506757977344,
    70377372123392,
    584921024674,
    35188734493778,
];

/// Magic numbers for the rook piece.
/// Do not modify this array. See the src/bin/generate_magics/main.rs for more information.
pub(crate) const ROOK_MAGIC_VALUES: [u64; NumberOf::SQUARES] = [
    6919781514650583040,
    4634204566689218560,
    2738228705771257856,
    288283238610272256,
    720611141964795904,
    576462985959048224,
    144117387746149376,
    4503755325384704,
    9242512475939799040,
    4613515880861532160,
    4755818877616193536,
    13836184026060423176,
    577588946796806144,
    20829183743627264,
    141016795586560,
    4611844417358200832,
    7061644765476945952,
    10520409005490176032,
    18016065099431937,
    4505799191824896,
    36029896799551492,
    6941318061255328780,
    360872918966095872,
    180144810005397504,
    279181262851,
    9799841723768111104,
    137976348672,
    648518488344764416,
    9263904450683338816,
    288230397627656192,
    68790919168,
    9223442407880724480,
    142271315968,
    9367488479732695040,
    68727898112,
    146454957560700928,
    4611686088356397056,
    1152921539512897536,
    9367771457278197760,
    562952109359104,
    1651474432001,
    576471889157832706,
    9223376469797899782,
    36031031695114240,
    433491273073295360,
    9277416331937382400,
    74309468107595776,
    1170935907432415232,
    72067629431522304,
    72637032238080,
    68786716688,
    36028953836274688,
    10376575257036210240,
    218426850207597568,
    102456900129915904,
    8613265552,
    9345932728387,
    2543700934788,
    690483648532,
    21764374562,
    162730002747716,
    45036014536753800,
    4601610372,
    613615449833242898,
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
            shift: shift,
            offset: offset,
            magic_value: magic_value,
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
        return ((blocker_num.wrapping_mul(self.magic_value) >> self.shift) + self.offset) as usize;
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
        let relevant_bits = MoveGenerator::relevant_rook_bits(Squares::A1 as usize);
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
