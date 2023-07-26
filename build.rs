use std::env;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let sources = [
        ("data/resources/assets/xkcd.txt", "XKCD", "XKCD_VALUES"),
        ("data/resources/assets/basic.txt", "BASIC", "BASIC_VALUES"),
        ("data/resources/assets/svg.txt", "SVG", "SVG_VALUES"),
        ("data/resources/assets/gnome.txt", "GNOME", "GNOME_VALUES"),
    ];

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).expect("Failed to create map file"));

    sources
        .iter()
        .inspect(|source| println!("cargo:rerun-if-changed={}", source.0))
        .for_each(|(path, name, rev_name)| {
            generate_map(&mut file, path, name, rev_name).expect("Failed to write map")
        });

    println!("cargo:rerun-if-changed=build.rs");
}

fn generate_map<T: AsRef<Path>>(
    file: &mut BufWriter<File>,
    path: T,
    name: &str,
    rev_name: &str,
) -> Result<(), io::Error> {
    let input_file = std::fs::read_to_string(path)?;
    let mut map = phf_codegen::Map::new();
    let mut reverse_map = phf_codegen::Map::new();

    input_file
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with("#"))
        .filter_map(|line| line.split_once(','))
        .map(|(name, val)| (name.trim(), val.trim()))
        .for_each(|(name, hex)| {
            map.entry(name, &format!("\"{}\"", hex));
            reverse_map.entry(hex, &format!("\"{}\"", name));
        });

    write_map(file, name, map)?;
    write_map(file, rev_name, reverse_map)
}

fn write_map(
    file: &mut BufWriter<File>,
    name: &str,
    map: phf_codegen::Map<&str>,
) -> Result<(), io::Error> {
    write!(
        file,
        "const {}: phf::Map<&'static str, &'static str> = {}",
        name,
        map.build()
    )?;
    write!(file, ";\n")
}
