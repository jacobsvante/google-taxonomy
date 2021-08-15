use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

// From http://www.google.com/basepages/producttype/taxonomy-with-ids.en-US.txt
static TAXONOMY_PATH: &str = "taxonomy-with-ids.en-US.txt";

fn variant_name(name: &str) -> String {
    name.replace("&", "And")
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric())
        .collect()
}

fn parse() -> Vec<(u32, String)> {
    let reader = BufReader::new(File::open(TAXONOMY_PATH).expect("Couldn't read data"));
    reader
        .lines()
        .skip(1)
        .into_iter()
        .enumerate()
        .map(|(line_no, line)| {
            let line =
                line.unwrap_or_else(|_| panic!("Problems reading line {} from text file", line_no));

            let columns: Vec<&str> = line.splitn(2, " - ").collect();

            (
                columns[0]
                    .parse()
                    .unwrap_or_else(|_| panic!("Could not parse id for {}", &line)),
                columns[1].into(),
            )
        })
        .collect()
}

#[rustfmt::skip]
fn write_enum(
    file: &mut BufWriter<File>,
    mut entries: Vec<(u32, String)>,
) -> Result<(), std::io::Error> {

    // The IDs seem to already be sorted alphabetically, but it's
    // required for our lookups (in lib.rs) to behave right.
    entries.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    // Also precalculate the order of the IDs to speed up runtime lookups
    let mut numerically_sorted = entries
        .iter()
        .enumerate()
        .map(|(idx, (id, _))| (id, idx))
        .collect::<Vec<_>>();

    numerically_sorted.sort_unstable_by(|a, b| a.0.cmp(b.0));

    // Emit the arrays
    writeln!(file, "const DATA: &[(u32, &str)] = &{:#?};", entries)?;
    writeln!(file, "const DATA_SORTED_BY_ID: &[(u32, u16)] = &{:#?};", numerically_sorted)?;

    // Declare constants for constructing each category
    writeln!(file, "#[allow(non_upper_case_globals)]")?;
    writeln!(file, "impl ProductCategory {{")?;
    writeln!(file)?;
    for (i, (_, name)) in entries.iter().enumerate() {
        writeln!(
            file,
            "    pub const {}: Self = Self({});",
            variant_name(name),
            i
        )?;
    }
    writeln!(file, "}}")?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("enum.rs");
    let entries = parse();
    let mut file = BufWriter::new(File::create(&out_path).expect("Couldn't write to output file"));
    write_enum(&mut file, entries)?;
    Ok(())
}
