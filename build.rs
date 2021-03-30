fn main() {
    nightly_lints();
    beta_lints();
}

#[rustversion::nightly]
fn nightly_lints() {
    println!("cargo:rustc-cfg=nightly_lints");
}

#[rustversion::not(nightly)]
fn nightly_lints() {}

#[rustversion::any(beta, nightly)]
fn beta_lints() {
    println!("cargo:rustc-cfg=beta_lints");
}

#[rustversion::stable]
fn beta_lints() {}
