// Note: make lookup table public in poker src in order to run

use std::{
    fs::File,
    io::{prelude::*, BufWriter, Result},
};

use poker::Evaluator;

fn main() -> Result<()> {
    let mut file = File::create("codegen.txt").map(BufWriter::new)?;

    let e = Evaluator::new();
    let five = e.five_card_table;
    let three = e.three_card_table;

    // Imports
    write!(&mut file, "use crate::evaluate::eval::Eval;\n\n")?;

    // Flush lookup (5 card)
    write!(
        &mut file,
        "pub static FLUSH_LOOKUP_FIVE: ::phf::Map<i32, Eval<crate::evaluate::poker_type::FiveCard>> = "
    )?;
    let mut builder = &mut phf_codegen::Map::new();
    for (&key, &value) in five.flush_lookup.iter() {
        builder = builder.entry(key, &format!("Eval::new({}, {})", value.hand_rank(), value.rank_flags()));
    }
    write!(&mut file, "{};\n\n", builder.build())?;

    // Unsuited lookup (5 card)
    write!(
        &mut file,
        "pub static UNSUITED_LOOKUP_FIVE: ::phf::Map<i32, Eval<crate::evaluate::poker_type::FiveCard>> = "
    )?;
    let mut builder = &mut phf_codegen::Map::new();
    for (&key, &value) in five.unsuited_lookup.iter() {
        builder = builder.entry(key, &format!("Eval::new({}, {})", value.hand_rank(), value.rank_flags()));
    }
    write!(&mut file, "{};\n\n", builder.build())?;

    // Flush lookup (3 card)
    write!(
        &mut file,
        "pub static FLUSH_LOOKUP_THREE: ::phf::Map<i32, Eval<crate::evaluate::poker_type::ThreeCard>> = "
    )?;
    let mut builder = &mut phf_codegen::Map::new();
    for (&key, &value) in three.flush_lookup.iter() {
        builder = builder.entry(key, &format!("Eval::new({}, {})", value.hand_rank(), value.rank_flags()));
    }
    write!(&mut file, "{};\n\n", builder.build())?;

    // Unsuited lookup (5 card)
    write!(
        &mut file,
        "pub static UNSUITED_LOOKUP_THREE: ::phf::Map<i32, Eval<crate::evaluate::poker_type::ThreeCard>> = "
    )?;
    let mut builder = &mut phf_codegen::Map::new();
    for (&key, &value) in three.unsuited_lookup.iter() {
        builder = builder.entry(key, &format!("Eval::new({}, {})", value.hand_rank(), value.rank_flags()));
    }
    write!(&mut file, "{};\n\n", builder.build())?;

    Ok(())
}
