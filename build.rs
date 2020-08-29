fn main() {
    let found_tls = cfg!(feature = "with_rustls") || cfg!(feature = "with_native_tls");
    if !found_tls {
        panic!(
            "slack: at least one of 'with_native_tls' or 'with_rustls' features must be enabled"
        );
    }
}
