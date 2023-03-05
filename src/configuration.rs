use config::{Config, Source};

pub fn get_db() {
    let file_name = "config.toml";
    let cf = config::File::with_name(file_name)
        .required(true)
        .format(config::FileFormat::Toml);

    println!("{:?}", cf.collect());
}
