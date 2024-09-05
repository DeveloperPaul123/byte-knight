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
    12105702255441815552,
    11961560614993788928,
    10380942349647085568,
    10070049067452661760,
    9655718264793202688,
    9583662206086545408,
    9570290014639898624,
    9460954795012784128,
    9263932059007848448,
    9259403035079610368,
    9241406346917586944,
    6990124351601281024,
    6918096379935989760,
    5764783857480105984,
    4611703894114840576,
    3497045218061459456,
    3459894881701601792,
    2920593197394562048,
    2918352349813702656,
    2522024724931936256,
    2310355542373498880,
    2306482942178230272,
    1170945799124705792,
    577615308248979456,
    576462985686484992,
    576461405756063744,
    378373016750522624,
    288234860101896512,
    288230397693657088,
    198159492243849984,
    144678249707996224,
    74318215991328912,
    72464421930401920,
    72092952129720448,
    72066416991817728,
    55315364975935496,
    36048590384398336,
    36028805894185984,
    18104559620718912,
    13511357363126274,
    9007474132976640,
    5092940553978400,
    4503875848013824,
    3377978904023040,
    2401475196092417,
    2393091487039649,
    2322173020774400,
    1127019837423632,
    879609874227202,
    589684245815300,
    299651815178752,
    19801981329664,
    18151072989696,
    17877801435136,
    17680233955594,
    9144539283520,
    2370830893344,
    2237988601857,
    2235547584544,
    2200420485128,
    1202876074241,
    1099645878912,
    559085387778,
    421043109888,
];

/// Magic numbers for the rook piece.
/// Do not modify this array. See the src/bin/generate_magics/main.rs for more information.
pub(crate) const ROOK_MAGIC_VALUES: [u64; NumberOf::SQUARES] = [
    288248389246723072,
    288252375515332608,
    2311472921201082368,
    2319353825276731392,
    288235332697079808,
    288234775917973504,
    27022156126750720,
    10954452010585358336,
    1301549161421864960,
    9225641450442588160,
    4785143877484544,
    19140333319503872,
    1176565505798971392,
    433471481851936768,
    11543288812100782080,
    36048041844678656,
    9259420626302959616,
    9799840486813925376,
    34429014400,
    15132096948095420416,
    27303218837454848,
    279745003520,
    9008573712574464,
    9367803901530738688,
    6917529201591451648,
    612490657428275200,
    58547624086605824,
    3458766867496206336,
    2882308176747704320,
    63050411965678096,
    9268409134789820416,
    1125902070087689,
    72057870031520288,
    216172919556931584,
    8813273939976,
    288230410519871488,
    1156303619562176512,
    9227875653682989056,
    74309411065167872,
    691758826496,
    1171094374625443840,
    576461855036342272,
    4629774221671989248,
    2323857730147844096,
    1157425173087977728,
    1407385086246912,
    40532465387569152,
    1299288493570228224,
    9223377019019067392,
    216177738512385024,
    18018833616868352,
    45038332907914240,
    72057907706856448,
    69260806144,
    25787107360,
    9244764139379769344,
    576463295201157248,
    9295430051800580096,
    137716048132,
    288806521326797824,
    34897281028,
    22518033234788868,
    5210664773263295488,
    109212292180615680,
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
