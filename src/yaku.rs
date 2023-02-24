use std::fmt::Display;

use crate::tile::{Dragon, Wind};

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Yaku {
    // 1-han
    Riichi,
    AllSimples,
    FullyConcealedHand,
    SeatWind(Wind),
    PrevalentWind(Wind),
    Dragons(Dragon),
    Pinfu,
    PureDoubleSequence,
    RobbingAKan,
    AfterAKan,
    UnderTheSea,
    UnderTheRiver,
    Ippatsu,
    TsubameGaeshi,
    Kanburi,
    Shiiatutaotai,

    // 2-han
    DoubeRiichi,
    TripleTriplets,
    ThreeQuads,
    AllTriplets,
    ThreeConcealedTriplets,
    LittleThreeDragons,
    AllTerminalsAndHonors,
    SevenPairs,
    HalfOutsideHand,
    PureStraight,
    MixedTripleSequence,
    Uumensai,
    ThreeChainedTriplets,

    // 3-han
    TwicePureDoubleSequence,
    FullyOutsideHand,
    HalfFlush,
    PureTripleChow,

    // 6-han
    FullFlush,

    // mangan
    ManganAtDraw,
    Iipinmoyue,
    Chuupinraoyui,

    // yakuman
    BlessingOfHeaven,
    BlessingOfEarth,
    BigThreeDragons,
    FourConcealedTriplets,
    AllHonors,
    AllGreen,
    AllTerminals,
    ThirteenOrphans,
    FourLittleWinds,
    FourQuads,
    NineGates,
    HandOfMan,
    BigWheels,
    BambooForest,
    NumerousNeighbours,
    Ishinouenimosannen,

    // double-yakuman
    SingleWaitFourConcealedTriplets,
    ThirteenWaitThirteenOrphans,
    TrueNineGates,
    FourBigWinds,
    BigSevenStars,
}

impl Display for Yaku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Yaku::Riichi => write!(f, "Riichi"),
            Yaku::AllSimples => write!(f, "All simples"),
            Yaku::FullyConcealedHand => write!(f, "Fully concealed hand"),
            Yaku::SeatWind(wind) => write!(
                f,
                "Seat wind: {}",
                match wind {
                    Wind::East => "East Wind",
                    Wind::South => "South Wind",
                    Wind::West => "West Wind",
                    Wind::North => "North Wind",
                }
            ),
            Yaku::PrevalentWind(wind) => write!(
                f,
                "Prevalent wind: {}",
                match wind {
                    Wind::East => "East Wind",
                    Wind::South => "South Wind",
                    Wind::West => "West Wind",
                    Wind::North => "North Wind",
                }
            ),
            Yaku::Dragons(dragon) => write!(
                f,
                "Dragons: {}",
                match dragon {
                    Dragon::Green => "Green Dragon",
                    Dragon::Red => "Red Dragon",
                    Dragon::White => "White Dragon",
                }
            ),
            Yaku::Pinfu => write!(f, "Pinfu"),
            Yaku::PureDoubleSequence => write!(f, "Pure double sequence"),
            Yaku::RobbingAKan => write!(f, "Robbing a Kan"),
            Yaku::AfterAKan => write!(f, "After a Kan"),
            Yaku::UnderTheSea => write!(f, "Under the sea"),
            Yaku::UnderTheRiver => write!(f, "Under the river"),
            Yaku::Ippatsu => write!(f, "Ippatsu"),
            Yaku::TsubameGaeshi => write!(f, "Tsubame Gaeshi"),
            Yaku::Kanburi => write!(f, "Kanburi"),
            Yaku::Shiiatutaotai => write!(f, "Shiiatutaotai"),
            Yaku::DoubeRiichi => write!(f, "Doube Riichi"),
            Yaku::TripleTriplets => write!(f, "Triple triplets"),
            Yaku::ThreeQuads => write!(f, "Three Quads"),
            Yaku::AllTriplets => write!(f, "All triplets"),
            Yaku::ThreeConcealedTriplets => {
                write!(f, "Three concealed triplets")
            }
            Yaku::LittleThreeDragons => write!(f, "Little three dragons"),
            Yaku::AllTerminalsAndHonors => {
                write!(f, "All terminals and honors")
            }
            Yaku::SevenPairs => write!(f, "Seven pairs"),
            Yaku::HalfOutsideHand => write!(f, "Half outside hand"),
            Yaku::PureStraight => write!(f, "Pure straight"),
            Yaku::MixedTripleSequence => write!(f, "Mixed triple sequence"),
            Yaku::Uumensai => write!(f, "Uumensai"),
            Yaku::ThreeChainedTriplets => write!(f, "Three chained triplets"),
            Yaku::TwicePureDoubleSequence => {
                write!(f, "Twice pure double sequence")
            }
            Yaku::FullyOutsideHand => write!(f, "Fully outside hand"),
            Yaku::HalfFlush => write!(f, "Half flush"),
            Yaku::PureTripleChow => write!(f, "Pure triple Chow"),
            Yaku::FullFlush => write!(f, "Full flush"),
            Yaku::ManganAtDraw => write!(f, "Mangan at draw"),
            Yaku::Iipinmoyue => write!(f, "Iipinmoyue"),
            Yaku::Chuupinraoyui => write!(f, "Chuupinraoyui"),
            Yaku::BlessingOfHeaven => write!(f, "Blessing of heaven"),
            Yaku::BlessingOfEarth => write!(f, "Blessing of earth"),
            Yaku::BigThreeDragons => write!(f, "Big three dragons"),
            Yaku::FourConcealedTriplets => write!(f, "Four concealed triplets"),
            Yaku::AllHonors => write!(f, "All honors"),
            Yaku::AllGreen => write!(f, "All green"),
            Yaku::AllTerminals => write!(f, "All terminals"),
            Yaku::ThirteenOrphans => write!(f, "Thirteen orphans"),
            Yaku::FourLittleWinds => write!(f, "Four little winds"),
            Yaku::FourQuads => write!(f, "Four quads"),
            Yaku::NineGates => write!(f, "Nine gates"),
            Yaku::HandOfMan => write!(f, "Hand of Man"),
            Yaku::BigWheels => write!(f, "Big wheels"),
            Yaku::BambooForest => write!(f, "Bamboo forest"),
            Yaku::NumerousNeighbours => write!(f, "Numerous neighbours"),
            Yaku::Ishinouenimosannen => write!(f, "Ishinouenimosannen"),
            Yaku::SingleWaitFourConcealedTriplets => {
                write!(f, "Single wait four concealed triplets")
            }
            Yaku::ThirteenWaitThirteenOrphans => {
                write!(f, "Thirteen wait thirteen orphans")
            }
            Yaku::TrueNineGates => write!(f, "True nine gates"),
            Yaku::FourBigWinds => write!(f, "Four big winds"),
            Yaku::BigSevenStars => write!(f, "Big seven stars"),
        }
    }
}
