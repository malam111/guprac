use std::ops::{Deref, DerefMut};
use std::fs::File;
use std::path::Path;
use std::error::Error;

use crate::units::{Interval, Direction};
//use super::{Scale};
use crate::units::Moves;

use serde::{Deserialize, Serialize};

pub struct ScaleTypes {
}

impl ScaleType {

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleType {
    name: String,
    steps: Vec<u8>,
    degree: Vec<u8>
}

//#[derive(Educe)]
//#[educe(Default)]
//pub enum ScaleType {
//    #[educe(Default)]
//    Ionian,
//    Dorian,
//    Phrygian,
//    Lydian,
//    Mixolydian,
//    Aeolian,
//    Locrian,
//
//    HarmonicMinor,
//    MelodicMinor,
//
//    PentatonicMajor,
//    PentatonicMinor,
//
//    Blues,
//    WholeTone,
//    Chromatic,
//}
//
//impl ScaleType {
//    pub fn get_moves(&self, direction: Direction) -> ScaleMoves {
//        ScaleMoves (
//        match self {
//            Self::Ionian => Moves::from_vec(vec![2, 2, 1, 2, 2, 2, 1]).unwrap(),
//            _ => vec![]
//        }
//        )
//    }
//}
//
//pub struct ScaleMoves(Vec<Moves>);
//
//impl Deref for ScaleMoves {
//    type Target = Vec<Moves>;
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
//}
//
//impl DerefMut for ScaleMoves {
//    fn deref_mut(&mut self) -> &mut Self::Target {
//        &mut self.0
//    }
//}

#[cfg(test)]
mod test {

    use super::*;
    
    #[test]
    fn scale_types_json_parse_str_test() -> serde_json::Result<()>{
        let data = r#"
            [
                {
                    "name": "Ionian",
                    "steps": [
                        2, 2, 1, 2, 2, 2, 1
                    ],
                    "degree": [
                        1, 2, 3, 4, 5, 6, 7
                    ]
                },
                {
                    "name": "Pentatonic Minor",
                    "steps": [
                        3, 2, 2, 3, 2
                    ],
                    "degree": [
                        1, 3, 4, 5, 7
                    ]
                }
            ]
        "#;        
        let left = vec![
            ScaleType {
                name: "Ionian".to_string(),
                steps: vec![2,2,1,2,2,2,1],
                degree: vec![1,2,3,4,5,6,7]
            },
            ScaleType {
                name: "Pentatonic Minor".to_string(),
                steps: vec![3,2,2,3,2],
                degree: vec![1,3,4,5,7]
            }
        ];

        let parsed: Vec<ScaleType> = serde_json::from_str(data)?;

        assert_eq!(left, parsed);
        Ok(())
    }

    #[test]
    fn scale_types_json_parse_file_test() -> Result<(), Box<dyn Error>> {
        let left = vec![
            ScaleType {
                name: "Ionian".to_string(),
                steps: vec![2,2,1,2,2,2,1],
                degree: vec![1,2,3,4,5,6,7]
            },
            ScaleType {
                name: "Pentatonic Minor".to_string(),
                steps: vec![3,2,2,3,2],
                degree: vec![1,3,4,5,7]
            }
        ];
        let file = File::open("scales.json")?;
        let scales: Vec<ScaleType> = serde_json::from_reader(file)?;
        assert_eq!(left, scales);
        Ok(())

    }
}
