// Note: make lookup table public in poker src in order to run

use std::{
    fs::File,
    io::{prelude::*, BufWriter, Result},
};

use poker::Evaluator;

fn main() -> Result<()> {
    let mut file = File::create("codegen.txt").map(BufWriter::new)?;

    let table = Evaluator::new().0;

    // Flush lookup
    write!(
        &mut file,
        "use crate::{{evaluate::{{hand_rank::PokerHandRank, meta::Meta::{{self, *}}}}, \
         Rank::*}};\n\npub static FLUSH_LOOKUP: ::phf::Map<i32, Meta> = "
    )?;
    let mut builder = &mut phf_codegen::Map::new();
    for (&key, &value) in table.flush_lookup.iter() {
        builder = builder.entry(key, &format!("{:?}", value));
    }
    write!(&mut file, "{};\n\n", builder.build())?;

    // Unsuited lookup
    write!(
        &mut file,
        "pub static UNSUITED_LOOKUP: ::phf::Map<i32, Meta> = "
    )?;
    let mut builder = &mut phf_codegen::Map::new();
    for (&key, &value) in table.unsuited_lookup.iter() {
        builder = builder.entry(key, &format!("{:?}", value));
    }
    write!(&mut file, "{};", builder.build())?;

    Ok(())
}
