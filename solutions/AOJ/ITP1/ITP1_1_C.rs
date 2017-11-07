#![cfg_attr(not(debug_assertions), no_main)]

#[macro_use]
extern crate porus;
prelude!(solve);

fn solve() -> Result<(), Error> {
    let (stdin, stdout) = (&mut io::stdin()?, &mut io::stdout()?);
    let (mut a, mut b): (usize, usize) = default();
    io::scan(stdin, (&mut a, &mut b))?;
    io::print(stdout, (a*b," ",(a+b)*2,"\n"))?;
    Ok(())
}