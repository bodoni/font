#[macro_use]
mod support;

use std::collections::BTreeSet;

use font::features::{Component, Sample};
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
            ("case", "DFLT", "DFLT", ""),
            ("case", "latn", "AZE ", ""),
            ("case", "latn", "CAT ", ""),
            ("case", "latn", "CRT ", ""),
            ("case", "latn", "DFLT", ""),
            ("case", "latn", "KAZ ", ""),
            ("case", "latn", "MOL ", ""),
            ("case", "latn", "ROM ", ""),
            ("case", "latn", "TAT ", ""),
            ("case", "latn", "TRK ", ""),
            ("ccmp", "DFLT", "DFLT", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], [[i, j], [323, 327, 328], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [[300–304, 306–30c, 323, 327], [f890–f895, f897, f898, f89b–f89d, f89f]], [[302], [300]], [[302], [301]], [[302], [303]], [[302], [309]], [[306], [300]], [[306], [301]], [[306], [303]], [[306], [309]], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "AZE ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], [[i, j], [323, 327, 328], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [[300–304, 306–30c, 323, 327], [f890–f895, f897, f898, f89b–f89d, f89f]], [[302], [300]], [[302], [301]], [[302], [303]], [[302], [309]], [[306], [300]], [[306], [301]], [[306], [303]], [[306], [309]], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "CAT ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], [[i, j], [323, 327, 328], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [[300–304, 306–30c, 323, 327], [f890–f895, f897, f898, f89b–f89d, f89f]], [[302], [300]], [[302], [301]], [[302], [303]], [[302], [309]], [[306], [300]], [[306], [301]], [[306], [303]], [[306], [309]], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "CRT ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], [[i, j], [323, 327, 328], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [[300–304, 306–30c, 323, 327], [f890–f895, f897, f898, f89b–f89d, f89f]], [[302], [300]], [[302], [301]], [[302], [303]], [[302], [309]], [[306], [300]], [[306], [301]], [[306], [303]], [[306], [309]], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "DFLT", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], [[i, j], [323, 327, 328], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [[300–304, 306–30c, 323, 327], [f890–f895, f897, f898, f89b–f89d, f89f]], [[302], [300]], [[302], [301]], [[302], [303]], [[302], [309]], [[306], [300]], [[306], [301]], [[306], [303]], [[306], [309]], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "KAZ ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], [[i, j], [323, 327, 328], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [[300–304, 306–30c, 323, 327], [f890–f895, f897, f898, f89b–f89d, f89f]], [[302], [300]], [[302], [301]], [[302], [303]], [[302], [309]], [[306], [300]], [[306], [301]], [[306], [303]], [[306], [309]], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "MOL ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], [[i, j], [323, 327, 328], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [[300–304, 306–30c, 323, 327], [f890–f895, f897, f898, f89b–f89d, f89f]], [[302], [300]], [[302], [301]], [[302], [303]], [[302], [309]], [[306], [300]], [[306], [301]], [[306], [303]], [[306], [309]], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "ROM ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], [[i, j], [323, 327, 328], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [[300–304, 306–30c, 323, 327], [f890–f895, f897, f898, f89b–f89d, f89f]], [[302], [300]], [[302], [301]], [[302], [303]], [[302], [309]], [[306], [300]], [[306], [301]], [[306], [303]], [[306], [309]], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "TAT ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], [[i, j], [323, 327, 328], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [[300–304, 306–30c, 323, 327], [f890–f895, f897, f898, f89b–f89d, f89f]], [[302], [300]], [[302], [301]], [[302], [303]], [[302], [309]], [[306], [300]], [[306], [301]], [[306], [303]], [[306], [309]], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("ccmp", "latn", "TRK ", "[[[A–Z, À–Ö, Ø–Þ, 100, 102, 104, 106, 108, 10a, 10c, 10e, 110, 112, 114, 116, 118, 11a, 11c, 11e, 120, 122, 124, 126, 128, 12a, 12c, 12e, 130, 132, 134, 136, 139, 13b, 13d, 13f, 141, 143, 145, 147, 14a, 14c, 14e, 150, 152, 154, 156, 158, 15a, 15c, 15e, 160, 162, 164, 166, 168, 16a, 16c, 16e, 170, 172, 174, 176, 178, 179, 17b, 17d, 189, 1a0, 1af, 1cd, 1cf, 1d1, 1d3, 1d7, 1d9, 1db, 1e2, 1fc, 1fe, 218, 21a, 232, 1e00, 1e02, 1e04, 1e06, 1e08, 1e0a, 1e0c, 1e0e, 1e10, 1e12, 1e18, 1e1a, 1e1e, 1e20, 1e22, 1e24, 1e26, 1e28, 1e2a, 1e2e, 1e30, 1e32, 1e34, 1e36, 1e38, 1e3a, 1e3c, 1e3e, 1e40, 1e42, 1e44, 1e46, 1e48, 1e4a, 1e4e, 1e54, 1e56, 1e58, 1e5a, 1e5e, 1e60, 1e62, 1e64, 1e66, 1e68, 1e6a, 1e6c, 1e6e, 1e70, 1e72, 1e74, 1e76, 1e7c, 1e7e, 1e80, 1e82, 1e84, 1e86, 1e88, 1e8a, 1e8c, 1e8e, 1e90, 1e92, 1e94, 1ea0, 1ea2, 1ea4, 1ea6, 1ea8, 1eaa, 1eac, 1eae, 1eb0, 1eb2, 1eb4, 1eb6, 1eb8, 1eba, 1ebc, 1ebe, 1ec0, 1ec2, 1ec4, 1ec6, 1ec8, 1eca, 1ecc, 1ece, 1ed0, 1ed2, 1ed4, 1ed6, 1ed8, 1eda, 1edc, 1ede, 1ee0, 1ee2, 1ee4, 1ee6, 1ee8, 1eea, 1eec, 1eee, 1ef0, 1ef2, 1ef4, 1ef6, 1ef8], [300–304, 306–30c, 323, 327]], [[i, j], [300–304, 306–30c, 312]], [[i, j], [323, 327, 328], [300–304, 306–30c, 312]], 13f, [[300–304, 306–30c, 323, 327], [300–304, 306–30c, 323, 327]], [[300–304, 306–30c, 323, 327], [f890–f895, f897, f898, f89b–f89d, f89f]], [[302], [300]], [[302], [301]], [[302], [303]], [[302], [309]], [[306], [300]], [[306], [301]], [[306], [303]], [[306], [309]], [[f890–f895, f897, f898, f89b–f89d, f89f], [300–304, 306–30c, 323, 327]], fb01, fb02]"),
            ("dlig", "DFLT", "DFLT", "[[[T], [h]], [[f], [b]], [[f], [f], [b]], [[f], [f], [h]], [[f], [f], [j]], [[f], [f], [k]], [[f], [h]], [[f], [j]], [[f], [k]]]"),
            ("dlig", "latn", "AZE ", "[[[T], [h]], [[f], [b]], [[f], [f], [b]], [[f], [f], [h]], [[f], [f], [j]], [[f], [f], [k]], [[f], [h]], [[f], [j]], [[f], [k]]]"),
            ("dlig", "latn", "CAT ", "[[[T], [h]], [[f], [b]], [[f], [f], [b]], [[f], [f], [h]], [[f], [f], [j]], [[f], [f], [k]], [[f], [h]], [[f], [j]], [[f], [k]]]"),
            ("dlig", "latn", "CRT ", "[[[T], [h]], [[f], [b]], [[f], [f], [b]], [[f], [f], [h]], [[f], [f], [j]], [[f], [f], [k]], [[f], [h]], [[f], [j]], [[f], [k]]]"),
            ("dlig", "latn", "DFLT", "[[[T], [h]], [[f], [b]], [[f], [f], [b]], [[f], [f], [h]], [[f], [f], [j]], [[f], [f], [k]], [[f], [h]], [[f], [j]], [[f], [k]]]"),
            ("dlig", "latn", "KAZ ", "[[[T], [h]], [[f], [b]], [[f], [f], [b]], [[f], [f], [h]], [[f], [f], [j]], [[f], [f], [k]], [[f], [h]], [[f], [j]], [[f], [k]]]"),
            ("dlig", "latn", "MOL ", "[[[T], [h]], [[f], [b]], [[f], [f], [b]], [[f], [f], [h]], [[f], [f], [j]], [[f], [f], [k]], [[f], [h]], [[f], [j]], [[f], [k]]]"),
            ("dlig", "latn", "ROM ", "[[[T], [h]], [[f], [b]], [[f], [f], [b]], [[f], [f], [h]], [[f], [f], [j]], [[f], [f], [k]], [[f], [h]], [[f], [j]], [[f], [k]]]"),
            ("dlig", "latn", "TAT ", "[[[T], [h]], [[f], [b]], [[f], [f], [b]], [[f], [f], [h]], [[f], [f], [j]], [[f], [f], [k]], [[f], [h]], [[f], [j]], [[f], [k]]]"),
            ("dlig", "latn", "TRK ", "[[[T], [h]], [[f], [b]], [[f], [f], [b]], [[f], [f], [h]], [[f], [f], [j]], [[f], [f], [k]], [[f], [h]], [[f], [j]], [[f], [k]]]"),
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
            ("frac", "DFLT", "DFLT", "[[[1], [/], [2]], [[1], [/], [4]], [[3], [/], [4]]]"),
            ("frac", "latn", "AZE ", "[[[1], [/], [2]], [[1], [/], [4]], [[3], [/], [4]]]"),
            ("frac", "latn", "CAT ", "[[[1], [/], [2]], [[1], [/], [4]], [[3], [/], [4]]]"),
            ("frac", "latn", "CRT ", "[[[1], [/], [2]], [[1], [/], [4]], [[3], [/], [4]]]"),
            ("frac", "latn", "DFLT", "[[[1], [/], [2]], [[1], [/], [4]], [[3], [/], [4]]]"),
            ("frac", "latn", "KAZ ", "[[[1], [/], [2]], [[1], [/], [4]], [[3], [/], [4]]]"),
            ("frac", "latn", "MOL ", "[[[1], [/], [2]], [[1], [/], [4]], [[3], [/], [4]]]"),
            ("frac", "latn", "ROM ", "[[[1], [/], [2]], [[1], [/], [4]], [[3], [/], [4]]]"),
            ("frac", "latn", "TAT ", "[[[1], [/], [2]], [[1], [/], [4]], [[3], [/], [4]]]"),
            ("frac", "latn", "TRK ", "[[[1], [/], [2]], [[1], [/], [4]], [[3], [/], [4]]]"),
            ("kern", "DFLT", "DFLT", ""),
            ("kern", "latn", "DFLT", ""),
            ("liga", "DFLT", "DFLT", "[[[f], [f]], [[f], [f], [i]], [[f], [f], [l]], [[f], [i]], [[f], [l]]]"),
            ("liga", "latn", "AZE ", "[[[f], [f]], [[f], [f], [i]], [[f], [f], [l]], [[f], [i]], [[f], [l]]]"),
            ("liga", "latn", "CAT ", "[[[f], [f]], [[f], [f], [i]], [[f], [f], [l]], [[f], [i]], [[f], [l]]]"),
            ("liga", "latn", "CRT ", "[[[f], [f]], [[f], [f], [i]], [[f], [f], [l]], [[f], [i]], [[f], [l]]]"),
            ("liga", "latn", "DFLT", "[[[f], [f]], [[f], [f], [i]], [[f], [f], [l]], [[f], [i]], [[f], [l]]]"),
            ("liga", "latn", "KAZ ", "[[[f], [f]], [[f], [f], [i]], [[f], [f], [l]], [[f], [i]], [[f], [l]]]"),
            ("liga", "latn", "MOL ", "[[[f], [f]], [[f], [f], [i]], [[f], [f], [l]], [[f], [i]], [[f], [l]]]"),
            ("liga", "latn", "ROM ", "[[[f], [f]], [[f], [f], [i]], [[f], [f], [l]], [[f], [i]], [[f], [l]]]"),
            ("liga", "latn", "TAT ", "[[[f], [f]], [[f], [f], [i]], [[f], [f], [l]], [[f], [i]], [[f], [l]]]"),
            ("liga", "latn", "TRK ", "[[[f], [f]], [[f], [f], [i]], [[f], [f], [l]], [[f], [i]], [[f], [l]]]"),
            ("locl", "latn", "AZE ", "[i]"),
            ("locl", "latn", "CAT ", "[[[L], [·], [L]], [[l], [·], [l]]]"),
            ("locl", "latn", "CRT ", "[i]"),
            ("locl", "latn", "KAZ ", "[i]"),
            ("locl", "latn", "MOL ", "[15e, 15f, 162, 163]"),
            ("locl", "latn", "ROM ", "[15e, 15f, 162, 163]"),
            ("locl", "latn", "TAT ", "[i]"),
            ("locl", "latn", "TRK ", "[i]"),
            ("mark", "DFLT", "DFLT", ""),
            ("mark", "latn", "DFLT", ""),
            ("mkmk", "DFLT", "DFLT", ""),
            ("mkmk", "latn", "DFLT", ""),
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
    #[rustfmt::skip]
    assert_eq!(
        entries,
        [
            ("aalt", "DFLT", "DFLT", ""),
            ("aalt", "cyrl", "DFLT", ""),
            ("aalt", "cyrl", "MKD ", ""),
            ("aalt", "cyrl", "SRB ", ""),
            ("aalt", "grek", "APPH", ""),
            ("aalt", "grek", "DFLT", ""),
            ("aalt", "grek", "IPPH", ""),
            ("aalt", "latn", "APPH", ""),
            ("aalt", "latn", "CAT ", ""),
            ("aalt", "latn", "DFLT", ""),
            ("aalt", "latn", "IPPH", ""),
            ("aalt", "latn", "MAH ", ""),
            ("aalt", "latn", "MOL ", ""),
            ("aalt", "latn", "NAV ", ""),
            ("aalt", "latn", "ROM ", ""),
            ("aalt", "thai", "DFLT", ""),
            ("ccmp", "DFLT", "DFLT", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "cyrl", "DFLT", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "cyrl", "MKD ", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
            ("ccmp", "cyrl", "SRB ", "[[[i, j], [300–304, 306–308, 30a–30c, 312]]]"),
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
            ("ccmp", "thai", "DFLT", ""),
            ("dist", "DFLT", "DFLT", ""),
            ("dist", "cyrl", "DFLT", ""),
            ("dist", "cyrl", "MKD ", ""),
            ("dist", "cyrl", "SRB ", ""),
            ("dist", "grek", "APPH", ""),
            ("dist", "grek", "DFLT", ""),
            ("dist", "grek", "IPPH", ""),
            ("dist", "latn", "APPH", ""),
            ("dist", "latn", "CAT ", ""),
            ("dist", "latn", "DFLT", ""),
            ("dist", "latn", "IPPH", ""),
            ("dist", "latn", "MAH ", ""),
            ("dist", "latn", "MOL ", ""),
            ("dist", "latn", "NAV ", ""),
            ("dist", "latn", "ROM ", ""),
            ("dist", "thai", "DFLT", ""),
            ("kern", "DFLT", "DFLT", ""),
            ("kern", "latn", "APPH", ""),
            ("kern", "latn", "CAT ", ""),
            ("kern", "latn", "DFLT", ""),
            ("kern", "latn", "IPPH", ""),
            ("kern", "latn", "MAH ", ""),
            ("kern", "latn", "MOL ", ""),
            ("kern", "latn", "NAV ", ""),
            ("kern", "latn", "ROM ", ""),
            ("kern", "thai", "DFLT", ""),
            ("liga", "DFLT", "DFLT", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "cyrl", "DFLT", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "cyrl", "MKD ", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "cyrl", "SRB ", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "grek", "APPH", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "grek", "DFLT", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "grek", "IPPH", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "latn", "APPH", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "latn", "CAT ", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "latn", "DFLT", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "latn", "IPPH", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "latn", "MAH ", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "latn", "MOL ", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "latn", "NAV ", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "latn", "ROM ", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("liga", "thai", "DFLT", "[[[e24], [e45]], [[e26], [e45]]]"),
            ("locl", "latn", "MOL ", "[15e, 15f]"),
            ("locl", "latn", "ROM ", "[15e, 15f]"),
            ("mark", "DFLT", "DFLT", ""),
            ("mark", "cyrl", "DFLT", ""),
            ("mark", "cyrl", "MKD ", ""),
            ("mark", "cyrl", "SRB ", ""),
            ("mark", "grek", "APPH", ""),
            ("mark", "grek", "DFLT", ""),
            ("mark", "grek", "IPPH", ""),
            ("mark", "latn", "APPH", ""),
            ("mark", "latn", "CAT ", ""),
            ("mark", "latn", "DFLT", ""),
            ("mark", "latn", "IPPH", ""),
            ("mark", "latn", "MAH ", ""),
            ("mark", "latn", "MOL ", ""),
            ("mark", "latn", "NAV ", ""),
            ("mark", "latn", "ROM ", ""),
            ("mark", "thai", "DFLT", ""),
            ("mkmk", "DFLT", "DFLT", ""),
            ("mkmk", "cyrl", "DFLT", ""),
            ("mkmk", "cyrl", "MKD ", ""),
            ("mkmk", "cyrl", "SRB ", ""),
            ("mkmk", "grek", "APPH", ""),
            ("mkmk", "grek", "DFLT", ""),
            ("mkmk", "grek", "IPPH", ""),
            ("mkmk", "latn", "APPH", ""),
            ("mkmk", "latn", "CAT ", ""),
            ("mkmk", "latn", "DFLT", ""),
            ("mkmk", "latn", "IPPH", ""),
            ("mkmk", "latn", "MAH ", ""),
            ("mkmk", "latn", "MOL ", ""),
            ("mkmk", "latn", "NAV ", ""),
            ("mkmk", "latn", "ROM ", ""),
            ("mkmk", "thai", "DFLT", ""),
            ("ordn", "DFLT", "DFLT", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "cyrl", "DFLT", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "cyrl", "MKD ", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "cyrl", "SRB ", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "grek", "APPH", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "grek", "DFLT", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "grek", "IPPH", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "latn", "APPH", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "latn", "CAT ", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "latn", "DFLT", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "latn", "IPPH", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "latn", "MAH ", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "latn", "MOL ", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "latn", "NAV ", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "latn", "ROM ", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ordn", "thai", "DFLT", "[[[0–9], [A, a]], [[0–9], [O, o]]]"),
            ("ss01", "DFLT", "DFLT", "[e0d, e10]"),
            ("ss01", "cyrl", "DFLT", "[e0d, e10]"),
            ("ss01", "cyrl", "MKD ", "[e0d, e10]"),
            ("ss01", "cyrl", "SRB ", "[e0d, e10]"),
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
    #[rustfmt::skip]
    assert_eq!(
        entries,
        [
            ("calt", "DFLT", "DFLT", ""),
            ("calt", "arab", "DFLT", ""),
            ("ccmp", "DFLT", "DFLT", "[622, …, 626, 628, …, 62c, 62e, 630, 632, 634, 636, 638, 63a, 641, 642, 646, 64a, 671, 679, 67e, 686, 688, 691, 698, 6a2, 6a4, 6a7, 6a9, 6af, 6c1, …, 6c3]"),
            ("ccmp", "arab", "DFLT", "[622, …, 626, 628, …, 62c, 62e, 630, 632, 634, 636, 638, 63a, 641, 642, 646, 64a, 671, 679, 67e, 686, 688, 691, 698, 6a2, 6a4, 6a7, 6a9, 6af, 6c1, …, 6c3]"),
            ("clig", "DFLT", "DFLT", ""),
            ("clig", "arab", "DFLT", ""),
            ("curs", "DFLT", "DFLT", ""),
            ("curs", "arab", "DFLT", ""),
            ("dnom", "DFLT", "DFLT", "[0, …, 9, 660, …, 669, 6f0, …, 6f9]"),
            ("dnom", "arab", "DFLT", "[0, …, 9, 660, …, 669, 6f0, …, 6f9]"),
            ("fina", "DFLT", "DFLT", "[627, 62d, 62f, 631, 633, 635, 637, 639, 643, …, 645, 647, …, 649, 66e, 66f, 6a1, 6ba, 6cc, 6d2, 8bb, …, 8bd]"),
            ("fina", "arab", "DFLT", "[627, 62d, 62f, 631, 633, 635, 637, 639, 643, …, 645, 647, …, 649, 66e, 66f, 6a1, 6ba, 6cc, 6d2, 8bb, …, 8bd]"),
            ("init", "DFLT", "DFLT", "[62d, 633, 635, 637, 639, 643, …, 645, 647, 649, 66e, 66f, 6a1, 6ba, 6cc, 8bb, …, 8bd]"),
            ("init", "arab", "DFLT", "[62d, 633, 635, 637, 639, 643, …, 645, 647, 649, 66e, 66f, 6a1, 6ba, 6cc, 8bb, …, 8bd]"),
            ("isol", "DFLT", "DFLT", "[6cc, 8bb, …, 8bd]"),
            ("isol", "arab", "DFLT", "[6cc, 8bb, …, 8bd]"),
            ("kern", "DFLT", "DFLT", ""),
            ("kern", "arab", "DFLT", ""),
            ("locl", "latn", "DFLT", "[20]"),
            ("mark", "DFLT", "DFLT", ""),
            ("mark", "arab", "DFLT", ""),
            ("medi", "DFLT", "DFLT", "[62d, 633, 635, 637, 639, 643, …, 645, 647, 649, 66e, 66f, 6a1, 6ba, 6cc, 8bb, …, 8bd]"),
            ("medi", "arab", "DFLT", "[62d, 633, 635, 637, 639, 643, …, 645, 647, 649, 66e, 66f, 6a1, 6ba, 6cc, 8bb, …, 8bd]"),
            ("numr", "DFLT", "DFLT", "[0, …, 9, 660, …, 669, 6f0, …, 6f9]"),
            ("numr", "arab", "DFLT", "[0, …, 9, 660, …, 669, 6f0, …, 6f9]"),
            ("onum", "DFLT", "DFLT", "[661, …, 664, 666, 669]"),
            ("onum", "arab", "DFLT", "[661, …, 664, 666, 669]"),
            ("rclt", "DFLT", "DFLT", ""),
            ("rclt", "arab", "DFLT", ""),
            ("salt", "DFLT", "DFLT", ""),
            ("salt", "arab", "DFLT", ""),
            ("salt", "latn", "DFLT", "[G (2), H (2), K (2), M (2), N (2), P (2), Q (2), R (3), U (2), Y (2)]"),
            ("ss01", "DFLT", "DFLT", ""),
            ("ss01", "arab", "DFLT", ""),
            ("ss02", "DFLT", "DFLT", ""),
            ("ss02", "arab", "DFLT", ""),
        ],
    );
}

fn extract<T>(font: &mut Font<T>) -> Vec<(String, String, String, String)>
where
    T: font::Read,
{
    let directory = ok!(font.features());
    let mut values = directory
        .scripts
        .iter()
        .flat_map(|(script, indices)| {
            indices
                .iter()
                .cloned()
                .map(|index| &directory.languages[index])
                .flat_map(|(language, indices)| {
                    indices
                        .iter()
                        .cloned()
                        .map(|index| &directory.features[index])
                        .map(|(feature, indices)| {
                            (
                                ok!(Tag::from(feature.clone()).as_str()).to_string(),
                                ok!(Tag::from(script.clone()).as_str()).to_string(),
                                ok!(Tag::from(language.clone()).as_str()).to_string(),
                                match indices
                                    .iter()
                                    .cloned()
                                    .map(|index| directory.samples[index].as_ref())
                                    .collect::<Option<Vec<_>>>()
                                {
                                    Some(samples) => flatten(
                                        &samples
                                            .into_iter()
                                            .flat_map(|values| {
                                                values.iter().flat_map(IntoIterator::into_iter)
                                            })
                                            .cloned()
                                            .collect(),
                                    ),
                                    _ => Default::default(),
                                },
                            )
                        })
                })
        })
        .collect::<Vec<_>>();
    values.sort();
    values
}

fn flatten(values: &BTreeSet<Sample>) -> String {
    let mut buffer = String::new();
    buffer.push('[');
    for (index, value) in values.iter().enumerate() {
        match value {
            Sample::Simple(Component::Scalar(value)) => {
                buffer.push_str(&escape(*value));
            }
            Sample::Simple(Component::Range((start, end))) => {
                buffer.push_str(&escape(*start));
                if *start as usize + 1 == *end as usize {
                    buffer.push_str(", ");
                } else {
                    buffer.push_str(", …, ");
                }
                buffer.push_str(&escape(*end));
            }
            Sample::Alternate((value, count)) => {
                buffer.push_str(&escape(*value));
                buffer.push_str(&format!(" ({count})"));
            }
            Sample::Compound(positions) => {
                buffer.push('[');
                for (index, components) in positions.iter().enumerate() {
                    buffer.push('[');
                    for (index, value) in components.iter().enumerate() {
                        match value {
                            Component::Scalar(value) => {
                                buffer.push_str(&escape(*value));
                            }
                            Component::Range((start, end)) => {
                                buffer.push_str(&escape(*start));
                                if *start as usize + 1 == *end as usize {
                                    buffer.push_str(", ");
                                } else {
                                    buffer.push('–');
                                }
                                buffer.push_str(&escape(*end));
                            }
                        }
                        if index + 1 < components.len() {
                            buffer.push_str(", ");
                        }
                    }
                    buffer.push(']');
                    if index + 1 < positions.len() {
                        buffer.push_str(", ");
                    }
                }
                buffer.push(']');
            }
        }
        if index + 1 < values.len() {
            buffer.push_str(", ");
        }
    }
    buffer.push(']');
    buffer
}

fn escape(value: char) -> String {
    if !value.is_control() && !value.is_whitespace() && (value as usize) < 0xFF {
        value.to_string()
    } else {
        format!("{:0x}", value as u32)
    }
}
