// run-pass
#![feature(f_strings)]

pub fn main() {
    // fill / alignment
    assert_eq!(f"Hello {"x":<5}!", "Hello x    !");
    assert_eq!(f"Hello {"x":-<5}!", "Hello x----!");
    assert_eq!(f"Hello {"x":^5}!", "Hello   x  !");
    assert_eq!(f"Hello {"x":>5}!", "Hello     x!");
    assert_eq!(f"Hello {"x":\>5}!", r#"Hello \\\\x!"#);

    // Sign / # / 0
    assert_eq!(f"Hello {5:+}!", "Hello +5!");
    assert_eq!(f"{27:#x}!", "0x1b!");
    assert_eq!(f"Hello {5:05}!",  "Hello 00005!");
    assert_eq!(f"Hello {-5:05}!", "Hello -0005!");
    assert_eq!(f"{27:#010x}!", "0x0000001b!");
}
