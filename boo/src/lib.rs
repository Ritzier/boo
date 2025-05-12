use nvim_oxi::{Dictionary, Function, Object};

#[nvim_oxi::plugin]
pub fn boo() -> Dictionary {
    println!("hi");

    let setup = Function::from_fn(boo_core::setup);
    let hi = Function::from_fn(boo_core::hi);

    let entries: [(&str, Object); 2] = [("setup", setup.into()), ("hi", hi.into())];

    Dictionary::from_iter(entries)
}
