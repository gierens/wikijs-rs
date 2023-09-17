#[allow(dead_code)]
fn mangen() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);

    let cmd = clap::Command::new("wikijs")
        .arg(clap::arg!(-u --url <URL>));
        // .arg(clap::arg!(-c --count <NUM>));

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("wikijs.1"), buffer)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    #[cfg(feature = "cli")]
    mangen()?;
    Ok(())
}
