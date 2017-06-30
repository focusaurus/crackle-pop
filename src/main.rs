// Full repo: https://github.com/focusaurus/crackle-pop

extern crate crackle_pop;

// Write a program that prints out the numbers 1 to 100 (inclusive). If the number is divisible by
// 3, print Crackle instead of the number. If it's divisible by 5, print Pop. If it's divisible by
// both 3 and 5, print CracklePop. You can use any language.

// I don’t wanna have my face on the cover of a Wheaties box.
// I wanna have my face on the cover of a Rice Crispies box:
// Snap, Crackle, Mitch, and Pop.
// ‘Hey, how the fuck did he do that?’
// Hey, in Hollywood, it’s all about who you know, and I know Crackle.
// – Mitch Hedberg

fn main() {
    // I like this version because it should use the iterator lazily and
    // if we had to print out many more numbers, the ram usage would be flatter I think
    for pop in (1..101).map(crackle_pop::snap) {
        println!("{}", pop);
    }
    // This version uses a bit more ram at once but makes fewer system calls.
    // Probably more effecient for small sizes but if we had huge sizes would use excessive ram.
    // let pops: Vec<String> = (1..101).map(crackle_pop::snap).collect();
    // println!("{}", pops.join("\n"));
}
