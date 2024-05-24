#[macro_use]
mod support;

use std::collections::BTreeSet;

use font::features::{Position, Sequence};
use font::opentype::truetype::Tag;
use font::Font;

use crate::support::{setup, Fixture};

#[test]
fn crimson_text() {
    let mut file = setup(Fixture::CrimsonText);
    let entries = extract(&mut file[0]);
    let entries = entries
        .iter()
        .map(|(feature, script, language, characters)| {
            (&**feature, &**script, &**language, &**characters)
        })
        .collect::<Vec<_>>();
    #[rustfmt::skip]
    assert_eq!(
        entries,
        [
            ("case", "DFLT", "DFLT", "[300, …, 304, 306, …, 30c, 323, 327]"),
            ("case", "latn", "AZE ", "[300, …, 304, 306, …, 30c, 323, 327]"),
            ("case", "latn", "CAT ", "[300, …, 304, 306, …, 30c, 323, 327]"),
            ("case", "latn", "CRT ", "[300, …, 304, 306, …, 30c, 323, 327]"),
            ("case", "latn", "DFLT", "[300, …, 304, 306, …, 30c, 323, 327]"),
            ("case", "latn", "KAZ ", "[300, …, 304, 306, …, 30c, 323, 327]"),
            ("case", "latn", "MOL ", "[300, …, 304, 306, …, 30c, 323, 327]"),
            ("case", "latn", "ROM ", "[300, …, 304, 306, …, 30c, 323, 327]"),
            ("case", "latn", "TAT ", "[300, …, 304, 306, …, 30c, 323, 327]"),
            ("case", "latn", "TRK ", "[300, …, 304, 306, …, 30c, 323, 327]"),
            ("ccmp", "DFLT", "DFLT", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [302, 300], [306, 300], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "AZE ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [302, 300], [306, 300], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "CAT ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [302, 300], [306, 300], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "CRT ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [302, 300], [306, 300], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "DFLT", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [302, 300], [306, 300], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "KAZ ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [302, 300], [306, 300], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "MOL ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [302, 300], [306, 300], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "ROM ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [302, 300], [306, 300], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "TAT ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [302, 300], [306, 300], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "TRK ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [302, 300], [306, 300], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("dlig", "DFLT", "DFLT", "[[T, h], [f, b]]"),
            ("dlig", "latn", "AZE ", "[[T, h], [f, b]]"),
            ("dlig", "latn", "CAT ", "[[T, h], [f, b]]"),
            ("dlig", "latn", "CRT ", "[[T, h], [f, b]]"),
            ("dlig", "latn", "DFLT", "[[T, h], [f, b]]"),
            ("dlig", "latn", "KAZ ", "[[T, h], [f, b]]"),
            ("dlig", "latn", "MOL ", "[[T, h], [f, b]]"),
            ("dlig", "latn", "ROM ", "[[T, h], [f, b]]"),
            ("dlig", "latn", "TAT ", "[[T, h], [f, b]]"),
            ("dlig", "latn", "TRK ", "[[T, h], [f, b]]"),
            ("dnom", "DFLT", "DFLT", "[0, …, 4]"),
            ("dnom", "latn", "AZE ", "[0, …, 4]"),
            ("dnom", "latn", "CAT ", "[0, …, 4]"),
            ("dnom", "latn", "CRT ", "[0, …, 4]"),
            ("dnom", "latn", "DFLT", "[0, …, 4]"),
            ("dnom", "latn", "KAZ ", "[0, …, 4]"),
            ("dnom", "latn", "MOL ", "[0, …, 4]"),
            ("dnom", "latn", "ROM ", "[0, …, 4]"),
            ("dnom", "latn", "TAT ", "[0, …, 4]"),
            ("dnom", "latn", "TRK ", "[0, …, 4]"),
            ("frac", "DFLT", "DFLT", "[[1, /, 2], [3, /, 4]]"),
            ("frac", "latn", "AZE ", "[[1, /, 2], [3, /, 4]]"),
            ("frac", "latn", "CAT ", "[[1, /, 2], [3, /, 4]]"),
            ("frac", "latn", "CRT ", "[[1, /, 2], [3, /, 4]]"),
            ("frac", "latn", "DFLT", "[[1, /, 2], [3, /, 4]]"),
            ("frac", "latn", "KAZ ", "[[1, /, 2], [3, /, 4]]"),
            ("frac", "latn", "MOL ", "[[1, /, 2], [3, /, 4]]"),
            ("frac", "latn", "ROM ", "[[1, /, 2], [3, /, 4]]"),
            ("frac", "latn", "TAT ", "[[1, /, 2], [3, /, 4]]"),
            ("frac", "latn", "TRK ", "[[1, /, 2], [3, /, 4]]"),
            ("kern", "DFLT", "DFLT", "[]"),
            ("kern", "latn", "DFLT", "[]"),
            ("liga", "DFLT", "DFLT", "[[f, f]]"),
            ("liga", "latn", "AZE ", "[[f, f]]"),
            ("liga", "latn", "CAT ", "[[f, f]]"),
            ("liga", "latn", "CRT ", "[[f, f]]"),
            ("liga", "latn", "DFLT", "[[f, f]]"),
            ("liga", "latn", "KAZ ", "[[f, f]]"),
            ("liga", "latn", "MOL ", "[[f, f]]"),
            ("liga", "latn", "ROM ", "[[f, f]]"),
            ("liga", "latn", "TAT ", "[[f, f]]"),
            ("liga", "latn", "TRK ", "[[f, f]]"),
            ("locl", "latn", "AZE ", "[i]"),
            ("locl", "latn", "CAT ", "[[L, ·, L], [l, ·, l]]"),
            ("locl", "latn", "CRT ", "[i]"),
            ("locl", "latn", "KAZ ", "[i]"),
            ("locl", "latn", "MOL ", "[15e, 15f, 162, 163]"),
            ("locl", "latn", "ROM ", "[15e, 15f, 162, 163]"),
            ("locl", "latn", "TAT ", "[i]"),
            ("locl", "latn", "TRK ", "[i]"),
            ("mark", "DFLT", "DFLT", "[]"),
            ("mark", "latn", "DFLT", "[]"),
            ("mkmk", "DFLT", "DFLT", "[]"),
            ("mkmk", "latn", "DFLT", "[]"),
            ("numr", "DFLT", "DFLT", "[0, …, 4]"),
            ("numr", "latn", "AZE ", "[0, …, 4]"),
            ("numr", "latn", "CAT ", "[0, …, 4]"),
            ("numr", "latn", "CRT ", "[0, …, 4]"),
            ("numr", "latn", "DFLT", "[0, …, 4]"),
            ("numr", "latn", "KAZ ", "[0, …, 4]"),
            ("numr", "latn", "MOL ", "[0, …, 4]"),
            ("numr", "latn", "ROM ", "[0, …, 4]"),
            ("numr", "latn", "TAT ", "[0, …, 4]"),
            ("numr", "latn", "TRK ", "[0, …, 4]"),
            ("sinf", "DFLT", "DFLT", "[0, …, 4]"),
            ("sinf", "latn", "AZE ", "[0, …, 4]"),
            ("sinf", "latn", "CAT ", "[0, …, 4]"),
            ("sinf", "latn", "CRT ", "[0, …, 4]"),
            ("sinf", "latn", "DFLT", "[0, …, 4]"),
            ("sinf", "latn", "KAZ ", "[0, …, 4]"),
            ("sinf", "latn", "MOL ", "[0, …, 4]"),
            ("sinf", "latn", "ROM ", "[0, …, 4]"),
            ("sinf", "latn", "TAT ", "[0, …, 4]"),
            ("sinf", "latn", "TRK ", "[0, …, 4]"),
            ("subs", "DFLT", "DFLT", "[0, …, 4]"),
            ("subs", "latn", "AZE ", "[0, …, 4]"),
            ("subs", "latn", "CAT ", "[0, …, 4]"),
            ("subs", "latn", "CRT ", "[0, …, 4]"),
            ("subs", "latn", "DFLT", "[0, …, 4]"),
            ("subs", "latn", "KAZ ", "[0, …, 4]"),
            ("subs", "latn", "MOL ", "[0, …, 4]"),
            ("subs", "latn", "ROM ", "[0, …, 4]"),
            ("subs", "latn", "TAT ", "[0, …, 4]"),
            ("subs", "latn", "TRK ", "[0, …, 4]"),
            ("sups", "DFLT", "DFLT", "[0, …, 4]"),
            ("sups", "latn", "AZE ", "[0, …, 4]"),
            ("sups", "latn", "CAT ", "[0, …, 4]"),
            ("sups", "latn", "CRT ", "[0, …, 4]"),
            ("sups", "latn", "DFLT", "[0, …, 4]"),
            ("sups", "latn", "KAZ ", "[0, …, 4]"),
            ("sups", "latn", "MOL ", "[0, …, 4]"),
            ("sups", "latn", "ROM ", "[0, …, 4]"),
            ("sups", "latn", "TAT ", "[0, …, 4]"),
            ("sups", "latn", "TRK ", "[0, …, 4]"),
            ("zero", "DFLT", "DFLT", "[0]"),
            ("zero", "latn", "AZE ", "[0]"),
            ("zero", "latn", "CAT ", "[0]"),
            ("zero", "latn", "CRT ", "[0]"),
            ("zero", "latn", "DFLT", "[0]"),
            ("zero", "latn", "KAZ ", "[0]"),
            ("zero", "latn", "MOL ", "[0]"),
            ("zero", "latn", "ROM ", "[0]"),
            ("zero", "latn", "TAT ", "[0]"),
            ("zero", "latn", "TRK ", "[0]"),
        ],
    );
}

#[test]
fn noto_serif() {
    let mut file = setup(Fixture::NotoSerifThai);
    let entries = extract(&mut file[0]);
    let entries = entries
        .iter()
        .map(|(feature, script, language, characters)| {
            (&**feature, &**script, &**language, &**characters)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        entries,
        [
            ("aalt", "cyrl", "DFLT", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "cyrl", "MKD ", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "cyrl", "SRB ", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "DFLT", "DFLT", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "grek", "APPH", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "grek", "DFLT", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "grek", "IPPH", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "latn", "APPH", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "latn", "CAT ", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "latn", "DFLT", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "latn", "IPPH", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "latn", "MAH ", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "latn", "MOL ", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "latn", "NAV ", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "latn", "ROM ", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("aalt", "thai", "DFLT", "[A, O, a, i, j, o, 15e, 15f, 331, e0d, …, e10, e24, e26, e2c, e31, e34, …, e3a, e47, …, e4d]"),
            ("ccmp", "cyrl", "DFLT", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "cyrl", "MKD ", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "cyrl", "SRB ", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "DFLT", "DFLT", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "grek", "APPH", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "grek", "DFLT", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "grek", "IPPH", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "latn", "APPH", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "latn", "CAT ", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "latn", "DFLT", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "latn", "IPPH", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "latn", "MAH ", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "latn", "MOL ", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "latn", "NAV ", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "latn", "ROM ", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "thai", "DFLT", "[[[i, j], [300–304, 306–308, 30a–30c, 312]], [[e0c, e0d, e12, e13], 331], [[e0d–e10, e24, e26], e38–e3a], [[e1b, e1d, e1f], [e31, e34–e37, e47–e4d]], [e2c, [e31, e34–e37, e47–e4e]], [[e31, e34–e37, e47–e4e], e48–e4c], e33, [e38, 331], [e39, 331]]"),
            ("dist", "cyrl", "DFLT", "[]"),
            ("dist", "cyrl", "MKD ", "[]"),
            ("dist", "cyrl", "SRB ", "[]"),
            ("dist", "DFLT", "DFLT", "[]"),
            ("dist", "grek", "APPH", "[]"),
            ("dist", "grek", "DFLT", "[]"),
            ("dist", "grek", "IPPH", "[]"),
            ("dist", "latn", "APPH", "[]"),
            ("dist", "latn", "CAT ", "[]"),
            ("dist", "latn", "DFLT", "[]"),
            ("dist", "latn", "IPPH", "[]"),
            ("dist", "latn", "MAH ", "[]"),
            ("dist", "latn", "MOL ", "[]"),
            ("dist", "latn", "NAV ", "[]"),
            ("dist", "latn", "ROM ", "[]"),
            ("dist", "thai", "DFLT", "[]"),
            ("kern", "DFLT", "DFLT", "[]"),
            ("kern", "latn", "APPH", "[]"),
            ("kern", "latn", "CAT ", "[]"),
            ("kern", "latn", "DFLT", "[]"),
            ("kern", "latn", "IPPH", "[]"),
            ("kern", "latn", "MAH ", "[]"),
            ("kern", "latn", "MOL ", "[]"),
            ("kern", "latn", "NAV ", "[]"),
            ("kern", "latn", "ROM ", "[]"),
            ("kern", "thai", "DFLT", "[]"),
            ("liga", "cyrl", "DFLT", "[[e24, e45], [e26, e45]]"),
            ("liga", "cyrl", "MKD ", "[[e24, e45], [e26, e45]]"),
            ("liga", "cyrl", "SRB ", "[[e24, e45], [e26, e45]]"),
            ("liga", "DFLT", "DFLT", "[[e24, e45], [e26, e45]]"),
            ("liga", "grek", "APPH", "[[e24, e45], [e26, e45]]"),
            ("liga", "grek", "DFLT", "[[e24, e45], [e26, e45]]"),
            ("liga", "grek", "IPPH", "[[e24, e45], [e26, e45]]"),
            ("liga", "latn", "APPH", "[[e24, e45], [e26, e45]]"),
            ("liga", "latn", "CAT ", "[[e24, e45], [e26, e45]]"),
            ("liga", "latn", "DFLT", "[[e24, e45], [e26, e45]]"),
            ("liga", "latn", "IPPH", "[[e24, e45], [e26, e45]]"),
            ("liga", "latn", "MAH ", "[[e24, e45], [e26, e45]]"),
            ("liga", "latn", "MOL ", "[[e24, e45], [e26, e45]]"),
            ("liga", "latn", "NAV ", "[[e24, e45], [e26, e45]]"),
            ("liga", "latn", "ROM ", "[[e24, e45], [e26, e45]]"),
            ("liga", "thai", "DFLT", "[[e24, e45], [e26, e45]]"),
            ("locl", "latn", "MOL ", "[15e, 15f]"),
            ("locl", "latn", "ROM ", "[15e, 15f]"),
            ("mark", "cyrl", "DFLT", "[]"),
            ("mark", "cyrl", "MKD ", "[]"),
            ("mark", "cyrl", "SRB ", "[]"),
            ("mark", "DFLT", "DFLT", "[]"),
            ("mark", "grek", "APPH", "[]"),
            ("mark", "grek", "DFLT", "[]"),
            ("mark", "grek", "IPPH", "[]"),
            ("mark", "latn", "APPH", "[]"),
            ("mark", "latn", "CAT ", "[]"),
            ("mark", "latn", "DFLT", "[]"),
            ("mark", "latn", "IPPH", "[]"),
            ("mark", "latn", "MAH ", "[]"),
            ("mark", "latn", "MOL ", "[]"),
            ("mark", "latn", "NAV ", "[]"),
            ("mark", "latn", "ROM ", "[]"),
            ("mark", "thai", "DFLT", "[]"),
            ("mkmk", "cyrl", "DFLT", "[]"),
            ("mkmk", "cyrl", "MKD ", "[]"),
            ("mkmk", "cyrl", "SRB ", "[]"),
            ("mkmk", "DFLT", "DFLT", "[]"),
            ("mkmk", "grek", "APPH", "[]"),
            ("mkmk", "grek", "DFLT", "[]"),
            ("mkmk", "grek", "IPPH", "[]"),
            ("mkmk", "latn", "APPH", "[]"),
            ("mkmk", "latn", "CAT ", "[]"),
            ("mkmk", "latn", "DFLT", "[]"),
            ("mkmk", "latn", "IPPH", "[]"),
            ("mkmk", "latn", "MAH ", "[]"),
            ("mkmk", "latn", "MOL ", "[]"),
            ("mkmk", "latn", "NAV ", "[]"),
            ("mkmk", "latn", "ROM ", "[]"),
            ("mkmk", "thai", "DFLT", "[]"),
            ("ordn", "cyrl", "DFLT", "[[0–9, [A, a]]]"),
            ("ordn", "cyrl", "MKD ", "[[0–9, [A, a]]]"),
            ("ordn", "cyrl", "SRB ", "[[0–9, [A, a]]]"),
            ("ordn", "DFLT", "DFLT", "[[0–9, [A, a]]]"),
            ("ordn", "grek", "APPH", "[[0–9, [A, a]]]"),
            ("ordn", "grek", "DFLT", "[[0–9, [A, a]]]"),
            ("ordn", "grek", "IPPH", "[[0–9, [A, a]]]"),
            ("ordn", "latn", "APPH", "[[0–9, [A, a]]]"),
            ("ordn", "latn", "CAT ", "[[0–9, [A, a]]]"),
            ("ordn", "latn", "DFLT", "[[0–9, [A, a]]]"),
            ("ordn", "latn", "IPPH", "[[0–9, [A, a]]]"),
            ("ordn", "latn", "MAH ", "[[0–9, [A, a]]]"),
            ("ordn", "latn", "MOL ", "[[0–9, [A, a]]]"),
            ("ordn", "latn", "NAV ", "[[0–9, [A, a]]]"),
            ("ordn", "latn", "ROM ", "[[0–9, [A, a]]]"),
            ("ordn", "thai", "DFLT", "[[0–9, [A, a]]]"),
            ("ss01", "cyrl", "DFLT", "[e0d, e10]"),
            ("ss01", "cyrl", "MKD ", "[e0d, e10]"),
            ("ss01", "cyrl", "SRB ", "[e0d, e10]"),
            ("ss01", "DFLT", "DFLT", "[e0d, e10]"),
            ("ss01", "grek", "APPH", "[e0d, e10]"),
            ("ss01", "grek", "DFLT", "[e0d, e10]"),
            ("ss01", "grek", "IPPH", "[e0d, e10]"),
            ("ss01", "latn", "APPH", "[e0d, e10]"),
            ("ss01", "latn", "CAT ", "[e0d, e10]"),
            ("ss01", "latn", "DFLT", "[e0d, e10]"),
            ("ss01", "latn", "IPPH", "[e0d, e10]"),
            ("ss01", "latn", "MAH ", "[e0d, e10]"),
            ("ss01", "latn", "MOL ", "[e0d, e10]"),
            ("ss01", "latn", "NAV ", "[e0d, e10]"),
            ("ss01", "latn", "ROM ", "[e0d, e10]"),
            ("ss01", "thai", "DFLT", "[e0d, e10]"),
        ],
    );
}

#[test]
fn qahiri() {
    let mut file = setup(Fixture::Qahiri);
    let entries = extract(&mut file[0]);
    let entries = entries
        .iter()
        .map(|(feature, script, language, characters)| {
            (&**feature, &**script, &**language, &**characters)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        entries,
        [
            ("calt", "arab", "DFLT", "[]"),
            ("calt", "DFLT", "DFLT", "[]"),
            ("ccmp", "arab", "DFLT", "[622, …, 626, 628, …, 62c, 62e, 630, 632, 634, 636, 638, 63a, 641, 642, 646, 64a, 671, 679, 67e, 686, 688, 691, 698, 6a2, 6a4, 6a7, 6a9, 6af, 6c1, …, 6c3]"),
            ("ccmp", "DFLT", "DFLT", "[622, …, 626, 628, …, 62c, 62e, 630, 632, 634, 636, 638, 63a, 641, 642, 646, 64a, 671, 679, 67e, 686, 688, 691, 698, 6a2, 6a4, 6a7, 6a9, 6af, 6c1, …, 6c3]"),
            ("clig", "arab", "DFLT", "[]"),
            ("clig", "DFLT", "DFLT", "[]"),
            ("curs", "arab", "DFLT", "[]"),
            ("curs", "DFLT", "DFLT", "[]"),
            ("dnom", "arab", "DFLT", "[0, …, 9, 660, …, 669, 6f0, …, 6f9]"),
            ("dnom", "DFLT", "DFLT", "[0, …, 9, 660, …, 669, 6f0, …, 6f9]"),
            ("fina", "arab", "DFLT", "[627, 62d, 62f, 631, 633, 635, 637, 639, 643, …, 645, 647, …, 649, 66e, 66f, 6a1, 6ba, 6cc, 6d2, 8bb, …, 8bd]"),
            ("fina", "DFLT", "DFLT", "[627, 62d, 62f, 631, 633, 635, 637, 639, 643, …, 645, 647, …, 649, 66e, 66f, 6a1, 6ba, 6cc, 6d2, 8bb, …, 8bd]"),
            ("init", "arab", "DFLT", "[62d, 633, 635, 637, 639, 643, …, 645, 647, 649, 66e, 66f, 6a1, 6ba, 6cc, 8bb, …, 8bd]"),
            ("init", "DFLT", "DFLT", "[62d, 633, 635, 637, 639, 643, …, 645, 647, 649, 66e, 66f, 6a1, 6ba, 6cc, 8bb, …, 8bd]"),
            ("isol", "arab", "DFLT", "[6cc, 8bb, …, 8bd]"),
            ("isol", "DFLT", "DFLT", "[6cc, 8bb, …, 8bd]"),
            ("kern", "arab", "DFLT", "[]"),
            ("kern", "DFLT", "DFLT", "[]"),
            ("locl", "latn", "DFLT", "[20]"),
            ("mark", "arab", "DFLT", "[]"),
            ("mark", "DFLT", "DFLT", "[]"),
            ("medi", "arab", "DFLT", "[62d, 633, 635, 637, 639, 643, …, 645, 647, 649, 66e, 66f, 6a1, 6ba, 6cc, 8bb, …, 8bd]"),
            ("medi", "DFLT", "DFLT", "[62d, 633, 635, 637, 639, 643, …, 645, 647, 649, 66e, 66f, 6a1, 6ba, 6cc, 8bb, …, 8bd]"),
            ("numr", "arab", "DFLT", "[0, …, 9, 660, …, 669, 6f0, …, 6f9]"),
            ("numr", "DFLT", "DFLT", "[0, …, 9, 660, …, 669, 6f0, …, 6f9]"),
            ("onum", "arab", "DFLT", "[661, …, 664, 666, 669]"),
            ("onum", "DFLT", "DFLT", "[661, …, 664, 666, 669]"),
            ("rclt", "arab", "DFLT", "[[20, 20]]"),
            ("rclt", "DFLT", "DFLT", "[[20, 20]]"),
            ("salt", "arab", "DFLT", "[627, 62d, 631, 633, 635, 639, 643, 645, 647, …, 649, 662, …, 664, 666, …, 668, 6a1]"),
            ("salt", "DFLT", "DFLT", "[627, 62d, 631, 633, 635, 639, 643, 645, 647, …, 649, 662, …, 664, 666, …, 668, 6a1]"),
            ("salt", "latn", "DFLT", "[G, H, K, M, N, P, …, R, U, Y]"),
            ("ss01", "arab", "DFLT", "[621, 654, 655]"),
            ("ss01", "DFLT", "DFLT", "[621, 654, 655]"),
            ("ss02", "arab", "DFLT", "[]"),
            ("ss02", "DFLT", "DFLT", "[]"),
        ],
    );
}

fn extract<T>(font: &mut Font<T>) -> Vec<(String, String, String, String)>
where
    T: font::Read,
{
    ok!(font.features())
        .into_iter()
        .flat_map(|(feature, value)| {
            value.into_iter().flat_map(move |(script, value)| {
                value.into_iter().map(move |(language, sequences)| {
                    (
                        ok!(Tag::from(feature.clone()).as_str()).to_string(),
                        ok!(Tag::from(script.clone()).as_str()).to_string(),
                        ok!(Tag::from(language).as_str()).to_string(),
                        flatten(&sequences),
                    )
                })
            })
        })
        .collect()
}

fn flatten(values: &BTreeSet<Sequence>) -> String {
    let mut buffer = String::new();
    buffer.push('[');
    for (index, value) in values.iter().enumerate() {
        flatten_sequence(value, &mut buffer);
        if index + 1 < values.len() {
            buffer.push_str(", ");
        }
    }
    buffer.push(']');
    buffer
}

fn flatten_sequence(value: &Sequence, buffer: &mut String) {
    match value {
        Sequence::Single(value) => {
            flatten_position(value, buffer);
        }
        Sequence::Range(start, end) => {
            buffer.push_str(&escape(*start));
            buffer.push_str(", …, ");
            buffer.push_str(&escape(*end));
        }
        Sequence::List(values) => {
            buffer.push('[');
            for (index, other) in values.iter().enumerate() {
                flatten_position(other, buffer);
                if index + 1 < values.len() {
                    buffer.push_str(", ");
                }
            }
            buffer.push(']');
        }
    }
}

fn flatten_position(value: &Position, buffer: &mut String) {
    match value {
        Position::Single(value) => {
            buffer.push_str(&escape(*value));
        }
        Position::Range(start, end) => {
            buffer.push_str(&escape(*start));
            buffer.push('–');
            buffer.push_str(&escape(*end));
        }
        Position::Set(values) => {
            buffer.push('[');
            for (index, other) in values.iter().enumerate() {
                flatten_position(other, buffer);
                if index + 1 < values.len() {
                    buffer.push_str(", ");
                }
            }
            buffer.push(']');
        }
    }
}

fn escape(value: char) -> String {
    if !value.is_control() && !value.is_whitespace() && (value as usize) < 0xFF {
        value.to_string()
    } else {
        format!("{:0x}", value as u32)
    }
}
