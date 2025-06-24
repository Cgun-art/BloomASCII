0𝕏1♜ꙮ⚡1꧂0𐌰1🜲𖣘1⛩𒀱⚙️0⓿༼0⚔️1⌁𓂀0➊✧1⟠0𝟎⩰🝐𝙃𐑓✖️🝫1𐰴⊕♤0✲🜸🞜1₁⏁☈✄𓅓🜟☯𒁳0➀✷⟁⛧1⇋☭➐1⚚⩵0⌬⟴💾➻⟙0🝪1⛬0𝕌𐊧⫷🝧1𝟙0𓆣⇌⛏🜨⟡🜋⚑✴️0╳⊹🜒𐂂1⚛⟁☸1🔆⛎0♮⚘⚕️𐂷𖼀✪
use rand::Rng;
use std::time::Duration;
use std::{thread, vec::Vec};

fn main() {
    const SIZE_IN_BYTES: usize = 1_880_000; // 1.88 MB

    loop {
        // Generate random data
        let mut data: Vec<u8> = Vec::with_capacity(SIZE_IN_BYTES);
        data.resize_with(SIZE_IN_BYTES, || rand::thread_rng().gen());

        // Optional: Print status
        println!("Generated 1.88MB of data.");

        // Optional: Pause to avoid 100% CPU usage
        thread::sleep(Duration::from_millis(100));
    }
}









[dependencies]
rand = "0.8"
