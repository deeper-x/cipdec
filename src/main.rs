use clap::Parser;

mod secure {
    pub mod cipher;
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    message: String,

    #[arg(short, long, default_value_t = 5)]
    shift: i64,
}

fn main() {
    // USAGE example:
    // cargo run -- --message="alberto de prezzo, è un ottimo ragazzo!" --shift=2

    let args = Args::parse();
    let msg = args.message;
    let shift = args.shift;

    let obj_m = secure::cipher::Message::new(shift);
    let enc = obj_m.encrypt(msg);

    let dec = obj_m.decrypt(&enc, shift);

    println!("{:?}", dec);
}
