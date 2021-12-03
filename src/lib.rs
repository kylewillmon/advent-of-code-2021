use std::env;
use std::fmt::Display;

pub use anyhow::Result;

pub fn aoc_main<InFilter, InType, PartOne, O1, PartTwo, O2>(
    day: u64,
    mut in_filter: InFilter,
    mut part_one: PartOne,
    mut part_two: PartTwo,
) -> Result<()>
where
    InType: Clone,
    InFilter: FnMut(&str) -> Result<InType>,
    PartOne: FnMut(InType) -> Result<O1>,
    PartTwo: FnMut(InType) -> Result<O2>,
    O1: Display,
    O2: Display,
{
    let input = input::puzzle_input(env::args(), day).unwrap();
    let input = in_filter(&input)?;
    println!("Part 1 Count: {}", part_one(input.clone())?);
    println!("Part 2 Count: {}", part_two(input)?);
    Ok(())
}

mod input {
    use super::Result;
    use anyhow::{anyhow, ensure};
    use std::env;
    use std::fs::{self, File};
    use std::io;
    use std::path::Path;

    fn file_arg(mut args: env::Args) -> Result<Option<String>> {
        ensure!(args.len() <= 2, "too many arguments");

        Ok(args.nth(1))
    }

    /// Read all bytes from a reader into a new String
    ///
    /// Replace with std::io::read_to_string() if that is ever stabilized
    fn read_to_string<R: io::Read>(reader: &mut R) -> io::Result<String> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        Ok(buf)
    }

    fn find_input_file<P: AsRef<Path>, Q: AsRef<Path>>(dir: P, subpath: Q) -> Result<File> {
        fn inner(dir: &Path, subpath: &Path) -> Result<File> {
            for dir in dir.ancestors() {
                let path = dir.join(subpath);
                if let Ok(f) = File::open(path) {
                    return Ok(f);
                }
            }
            Err(anyhow!("no input file found"))
        }
        inner(dir.as_ref(), subpath.as_ref())
    }

    pub fn puzzle_input(args: env::Args, day: u64) -> Result<String> {
        let input = match file_arg(args)? {
            Some(filename) => {
                if filename == "-" {
                    let mut stdin = io::stdin();
                    read_to_string(&mut stdin)?
                } else {
                    fs::read_to_string(filename)?
                }
            }
            None => {
                let mut file =
                    find_input_file(env::current_dir()?, format!("input/day{}.txt", day))?;
                read_to_string(&mut file)?
            }
        };
        Ok(input)
    }
}
