use std::ops::{Deref, DerefMut};
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::marker::PhantomData;

use crate::units::{Interval, Direction};
//use super::{Scale};
use crate::units::Moves;

use serde::{Deserialize, Serialize};

struct ScaleTypeBuilderPopulated;
struct ScaleTypeBuilderUnpopulated;

pub struct ScaleTypeBuilder<State> {
    scale_types: Vec<ScaleType>,
    _state: PhantomData<State> 
}

impl<T> ScaleTypeBuilder<T> {
    fn new() -> ScaleTypeBuilder<ScaleTypeBuilderUnpopulated> {
        ScaleTypeBuilder {
            scale_types: vec!(),
            _state: PhantomData::<ScaleTypeBuilderUnpopulated>
        }
    }
}

impl ScaleTypeBuilder<ScaleTypeBuilderUnpopulated> {

    fn path<P: AsRef<Path>>(self, path: P) -> Result<ScaleTypeBuilder<ScaleTypeBuilderPopulated>, Box<dyn Error>> {
        let file = File::open(path)?;
        Ok(ScaleTypeBuilder::<ScaleTypeBuilderPopulated> {
            _state: PhantomData::<ScaleTypeBuilderPopulated>,
            scale_types: serde_json::from_reader(file)?
        })
    }
}

impl ScaleTypeBuilder<ScaleTypeBuilderPopulated> {

    fn path<P: AsRef<Path>>(&mut self, path: P) -> Result<&mut ScaleTypeBuilder<ScaleTypeBuilderPopulated>, Box<dyn Error>> {
        let file = File::open(path)?;
        self.scale_types.append(&mut serde_json::from_reader::<_, Vec<ScaleType>>(file)?);
        Ok(self)
    }

    fn get(&self, scale_name: &str) -> Option<&ScaleType> {
        self.scale_types.get(0)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleType {
    name: String,
    // FIXME: add getter
    pub steps: Vec<u8>,
    degree: Vec<u8>
}

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
