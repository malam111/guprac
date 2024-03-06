mod units;
mod chords;
mod scales;
mod fe;

#[macro_use] extern crate educe;

#[cfg(test)]
mod test {

    use std::{thread, time::Duration};
    use std::ops::Deref;

    use super::fe::{NoteFeeder};
    use super::scales::{Scale, ScaleChunk};
    use super::units::Octave;

    #[test]
    fn lib_random_note() {
        let feeder = NoteFeeder::new().build();
        for (idx, i) in feeder.enumerate() {
            if idx > 2 { break }
            thread::sleep(Duration::from_secs(2));
            eprintln!("{}", i);
        }
        assert!(true);
    }

    #[test]
    fn lib_bulk_note() {
        let mut feeder = NoteFeeder::new().build();
        for i in feeder.bulk(20).into_iter() {
            eprintln!("{}, ", i);
        }
        assert!(true);
    }

    #[test]
    fn lib_major_note() {
        let scale = Scale::new().build(); 
        eprintln!("{:?}", scale);
        for (idx, n) in scale.into_iter().enumerate() {
            if idx > 36 { break }
            if idx > 8 { 
                 
            }
            eprintln!("{}", n);
        }
        assert!(true);
    }

    #[test]
    fn lib_major_bulk() {
        let scale = Scale::new().low(Octave::O3).high(Octave::O5).build();
        for n in scale.into_iter().collect::<ScaleChunk>().iter() {
            eprintln!("{}", n);
        }
        assert!(true);
    }

    #[test]
    fn lib_major_degree() {
        let scale = Scale::new().low(Octave::O3).high(Octave::O5).degree(true).build();
        for n in scale.into_iter().collect::<ScaleChunk>().iter() {
            eprintln!("{}", n);
        }
        assert!(true);
    }
}
