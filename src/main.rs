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
    let pops: Vec<String> = (1..101).map(crackle_pop::format).collect();
    println!("{}", pops.join("\n"));
}
