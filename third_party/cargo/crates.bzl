"""
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""

def _new_http_archive(name, **kwargs):
    if not native.existing_rule(name):
        native.new_http_archive(name=name, **kwargs)

def _new_git_repository(name, **kwargs):
    if not native.existing_rule(name):
        native.new_git_repository(name=name, **kwargs)

def raze_fetch_remote_crates():

    _new_http_archive(
        name = "raze__adler32__1_0_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/adler32/adler32-1.0.3.crate",
        type = "tar.gz",
        sha256 = "7e522997b529f05601e05166c07ed17789691f562762c7f3b987263d2dedee5c",
        strip_prefix = "adler32-1.0.3",
        build_file = "//third_party/cargo/remote:adler32-1.0.3.BUILD"
    )

    _new_http_archive(
        name = "raze__aho_corasick__0_6_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.6.8.crate",
        type = "tar.gz",
        sha256 = "68f56c7353e5a9547cbd76ed90f7bb5ffc3ba09d4ea9bd1d8c06c8b1142eeb5a",
        strip_prefix = "aho-corasick-0.6.8",
        build_file = "//third_party/cargo/remote:aho-corasick-0.6.8.BUILD"
    )

    _new_http_archive(
        name = "raze__allan__0_2_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/allan/allan-0.2.4.crate",
        type = "tar.gz",
        sha256 = "62ed9db31078b3c9e56ce77857fa21f6bdb062988c24a5c989c3f44fa1317b47",
        strip_prefix = "allan-0.2.4",
        build_file = "//third_party/cargo/remote:allan-0.2.4.BUILD"
    )

    _new_http_archive(
        name = "raze__ansi_term__0_11_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ansi_term/ansi_term-0.11.0.crate",
        type = "tar.gz",
        sha256 = "ee49baf6cb617b853aa8d93bf420db2383fab46d314482ca2803b40d5fde979b",
        strip_prefix = "ansi_term-0.11.0",
        build_file = "//third_party/cargo/remote:ansi_term-0.11.0.BUILD"
    )

    _new_http_archive(
        name = "raze__approx__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/approx/approx-0.1.1.crate",
        type = "tar.gz",
        sha256 = "08abcc3b4e9339e33a3d0a5ed15d84a687350c05689d825e0f6655eef9e76a94",
        strip_prefix = "approx-0.1.1",
        build_file = "//third_party/cargo/remote:approx-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__arrayvec__0_3_25",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/arrayvec/arrayvec-0.3.25.crate",
        type = "tar.gz",
        sha256 = "06f59fe10306bb78facd90d28c2038ad23ffaaefa85bac43c8a434cde383334f",
        strip_prefix = "arrayvec-0.3.25",
        build_file = "//third_party/cargo/remote:arrayvec-0.3.25.BUILD"
    )

    _new_http_archive(
        name = "raze__arrayvec__0_4_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/arrayvec/arrayvec-0.4.7.crate",
        type = "tar.gz",
        sha256 = "a1e964f9e24d588183fcb43503abda40d288c8657dfc27311516ce2f05675aef",
        strip_prefix = "arrayvec-0.4.7",
        build_file = "//third_party/cargo/remote:arrayvec-0.4.7.BUILD"
    )

    _new_http_archive(
        name = "raze__ascii__0_7_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ascii/ascii-0.7.1.crate",
        type = "tar.gz",
        sha256 = "3ae7d751998c189c1d4468cf0a39bb2eae052a9c58d50ebb3b9591ee3813ad50",
        strip_prefix = "ascii-0.7.1",
        build_file = "//third_party/cargo/remote:ascii-0.7.1.BUILD"
    )

    _new_http_archive(
        name = "raze__atom__0_3_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atom/atom-0.3.5.crate",
        type = "tar.gz",
        sha256 = "3c86699c3f02778ec07158376991c8f783dd1f2f95c579ffaf0738dc984b2fe2",
        strip_prefix = "atom-0.3.5",
        build_file = "//third_party/cargo/remote:atom-0.3.5.BUILD"
    )

    _new_http_archive(
        name = "raze__atty__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atty/atty-0.2.11.crate",
        type = "tar.gz",
        sha256 = "9a7d5b8723950951411ee34d271d99dddcc2035a16ab25310ea2c8cfd4369652",
        strip_prefix = "atty-0.2.11",
        build_file = "//third_party/cargo/remote:atty-0.2.11.BUILD"
    )

    _new_http_archive(
        name = "raze__backtrace__0_3_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/backtrace/backtrace-0.3.9.crate",
        type = "tar.gz",
        sha256 = "89a47830402e9981c5c41223151efcced65a0510c13097c769cede7efb34782a",
        strip_prefix = "backtrace-0.3.9",
        build_file = "//third_party/cargo/remote:backtrace-0.3.9.BUILD"
    )

    _new_http_archive(
        name = "raze__backtrace_sys__0_1_24",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/backtrace-sys/backtrace-sys-0.1.24.crate",
        type = "tar.gz",
        sha256 = "c66d56ac8dabd07f6aacdaf633f4b8262f5b3601a810a0dcddffd5c22c69daa0",
        strip_prefix = "backtrace-sys-0.1.24",
        build_file = "//third_party/cargo/remote:backtrace-sys-0.1.24.BUILD"
    )

    _new_http_archive(
        name = "raze__bindgen__0_35_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bindgen/bindgen-0.35.0.crate",
        type = "tar.gz",
        sha256 = "b023955126e7909ab9fc1d1973965b8b004f1f388afb5c589640ab483b3b0ad2",
        strip_prefix = "bindgen-0.35.0",
        build_file = "//third_party/cargo/remote:bindgen-0.35.0.BUILD"
    )

    _new_http_archive(
        name = "raze__bitflags__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-0.7.0.crate",
        type = "tar.gz",
        sha256 = "aad18937a628ec6abcd26d1489012cc0e18c21798210f491af69ded9b881106d",
        strip_prefix = "bitflags-0.7.0",
        build_file = "//third_party/cargo/remote:bitflags-0.7.0.BUILD"
    )

    _new_http_archive(
        name = "raze__bitflags__1_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-1.0.4.crate",
        type = "tar.gz",
        sha256 = "228047a76f468627ca71776ecdebd732a3423081fcf5125585bcd7c49886ce12",
        strip_prefix = "bitflags-1.0.4",
        build_file = "//third_party/cargo/remote:bitflags-1.0.4.BUILD"
    )

    _new_http_archive(
        name = "raze__byteorder__0_4_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/byteorder/byteorder-0.4.2.crate",
        type = "tar.gz",
        sha256 = "96c8b41881888cc08af32d47ac4edd52bc7fa27fef774be47a92443756451304",
        strip_prefix = "byteorder-0.4.2",
        build_file = "//third_party/cargo/remote:byteorder-0.4.2.BUILD"
    )

    _new_http_archive(
        name = "raze__byteorder__1_2_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/byteorder/byteorder-1.2.6.crate",
        type = "tar.gz",
        sha256 = "90492c5858dd7d2e78691cfb89f90d273a2800fc11d98f60786e5d87e2f83781",
        strip_prefix = "byteorder-1.2.6",
        build_file = "//third_party/cargo/remote:byteorder-1.2.6.BUILD"
    )

    _new_http_archive(
        name = "raze__bytes__0_4_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bytes/bytes-0.4.10.crate",
        type = "tar.gz",
        sha256 = "0ce55bd354b095246fc34caf4e9e242f5297a7fd938b090cadfea6eee614aa62",
        strip_prefix = "bytes-0.4.10",
        build_file = "//third_party/cargo/remote:bytes-0.4.10.BUILD"
    )

    _new_http_archive(
        name = "raze__cargo_metadata__0_6_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cargo_metadata/cargo_metadata-0.6.0.crate",
        type = "tar.gz",
        sha256 = "2d6809b327f87369e6f3651efd2c5a96c49847a3ed2559477ecba79014751ee1",
        strip_prefix = "cargo_metadata-0.6.0",
        build_file = "//third_party/cargo/remote:cargo_metadata-0.6.0.BUILD"
    )

    _new_http_archive(
        name = "raze__cc__1_0_25",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cc/cc-1.0.25.crate",
        type = "tar.gz",
        sha256 = "f159dfd43363c4d08055a07703eb7a3406b0dac4d0584d96965a3262db3c9d16",
        strip_prefix = "cc-1.0.25",
        build_file = "//third_party/cargo/remote:cc-1.0.25.BUILD"
    )

    _new_http_archive(
        name = "raze__cexpr__0_2_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cexpr/cexpr-0.2.3.crate",
        type = "tar.gz",
        sha256 = "42aac45e9567d97474a834efdee3081b3c942b2205be932092f53354ce503d6c",
        strip_prefix = "cexpr-0.2.3",
        build_file = "//third_party/cargo/remote:cexpr-0.2.3.BUILD"
    )

    _new_http_archive(
        name = "raze__cfg_if__0_1_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.5.crate",
        type = "tar.gz",
        sha256 = "0c4e7bb64a8ebb0d856483e1e682ea3422f883c5f5615a90d51a2c82fe87fdd3",
        strip_prefix = "cfg-if-0.1.5",
        build_file = "//third_party/cargo/remote:cfg-if-0.1.5.BUILD"
    )

    _new_http_archive(
        name = "raze__cgmath__0_16_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cgmath/cgmath-0.16.1.crate",
        type = "tar.gz",
        sha256 = "64a4b57c8f4e3a2e9ac07e0f6abc9c24b6fc9e1b54c3478cfb598f3d0023e51c",
        strip_prefix = "cgmath-0.16.1",
        build_file = "//third_party/cargo/remote:cgmath-0.16.1.BUILD"
    )

    _new_http_archive(
        name = "raze__chrono__0_2_25",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/chrono/chrono-0.2.25.crate",
        type = "tar.gz",
        sha256 = "9213f7cd7c27e95c2b57c49f0e69b1ea65b27138da84a170133fd21b07659c00",
        strip_prefix = "chrono-0.2.25",
        build_file = "//third_party/cargo/remote:chrono-0.2.25.BUILD"
    )

    _new_http_archive(
        name = "raze__chrono__0_4_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/chrono/chrono-0.4.6.crate",
        type = "tar.gz",
        sha256 = "45912881121cb26fad7c38c17ba7daa18764771836b34fab7d3fbd93ed633878",
        strip_prefix = "chrono-0.4.6",
        build_file = "//third_party/cargo/remote:chrono-0.4.6.BUILD"
    )

    _new_http_archive(
        name = "raze__chunked_transfer__0_3_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/chunked_transfer/chunked_transfer-0.3.1.crate",
        type = "tar.gz",
        sha256 = "498d20a7aaf62625b9bf26e637cf7736417cde1d0c99f1d04d1170229a85cf87",
        strip_prefix = "chunked_transfer-0.3.1",
        build_file = "//third_party/cargo/remote:chunked_transfer-0.3.1.BUILD"
    )

    _new_http_archive(
        name = "raze__clang_sys__0_22_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clang-sys/clang-sys-0.22.0.crate",
        type = "tar.gz",
        sha256 = "939a1a34310b120d26eba35c29475933128b0ec58e24b43327f8dbe6036fc538",
        strip_prefix = "clang-sys-0.22.0",
        build_file = "//third_party/cargo/remote:clang-sys-0.22.0.BUILD"
    )

    _new_http_archive(
        name = "raze__clap__2_32_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clap/clap-2.32.0.crate",
        type = "tar.gz",
        sha256 = "b957d88f4b6a63b9d70d5f454ac8011819c6efa7727858f458ab71c756ce2d3e",
        strip_prefix = "clap-2.32.0",
        build_file = "//third_party/cargo/remote:clap-2.32.0.BUILD"
    )

    _new_http_archive(
        name = "raze__clocksource__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clocksource/clocksource-0.4.0.crate",
        type = "tar.gz",
        sha256 = "55ee428f7db909b905a58e4060c4f7318cd403641955b8b564a1220b2a5acd21",
        strip_prefix = "clocksource-0.4.0",
        build_file = "//third_party/cargo/remote:clocksource-0.4.0.BUILD"
    )

    _new_http_archive(
        name = "raze__cloudabi__0_0_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cloudabi/cloudabi-0.0.3.crate",
        type = "tar.gz",
        sha256 = "ddfc5b9aa5d4507acaf872de71051dfd0e309860e88966e1051e462a077aac4f",
        strip_prefix = "cloudabi-0.0.3",
        build_file = "//third_party/cargo/remote:cloudabi-0.0.3.BUILD"
    )

    _new_http_archive(
        name = "raze__crossbeam__0_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam/crossbeam-0.3.2.crate",
        type = "tar.gz",
        sha256 = "24ce9782d4d5c53674646a6a4c1863a21a8fc0cb649b3c94dfc16e45071dea19",
        strip_prefix = "crossbeam-0.3.2",
        build_file = "//third_party/cargo/remote:crossbeam-0.3.2.BUILD"
    )

    _new_http_archive(
        name = "raze__crossbeam_deque__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-deque/crossbeam-deque-0.2.0.crate",
        type = "tar.gz",
        sha256 = "f739f8c5363aca78cfb059edf753d8f0d36908c348f3d8d1503f03d8b75d9cf3",
        strip_prefix = "crossbeam-deque-0.2.0",
        build_file = "//third_party/cargo/remote:crossbeam-deque-0.2.0.BUILD"
    )

    _new_http_archive(
        name = "raze__crossbeam_deque__0_6_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-deque/crossbeam-deque-0.6.1.crate",
        type = "tar.gz",
        sha256 = "3486aefc4c0487b9cb52372c97df0a48b8c249514af1ee99703bf70d2f2ceda1",
        strip_prefix = "crossbeam-deque-0.6.1",
        build_file = "//third_party/cargo/remote:crossbeam-deque-0.6.1.BUILD"
    )

    _new_http_archive(
        name = "raze__crossbeam_epoch__0_3_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-epoch/crossbeam-epoch-0.3.1.crate",
        type = "tar.gz",
        sha256 = "927121f5407de9956180ff5e936fe3cf4324279280001cd56b669d28ee7e9150",
        strip_prefix = "crossbeam-epoch-0.3.1",
        build_file = "//third_party/cargo/remote:crossbeam-epoch-0.3.1.BUILD"
    )

    _new_http_archive(
        name = "raze__crossbeam_epoch__0_5_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-epoch/crossbeam-epoch-0.5.2.crate",
        type = "tar.gz",
        sha256 = "30fecfcac6abfef8771151f8be4abc9e4edc112c2bcb233314cafde2680536e9",
        strip_prefix = "crossbeam-epoch-0.5.2",
        build_file = "//third_party/cargo/remote:crossbeam-epoch-0.5.2.BUILD"
    )

    _new_http_archive(
        name = "raze__crossbeam_utils__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-utils/crossbeam-utils-0.2.2.crate",
        type = "tar.gz",
        sha256 = "2760899e32a1d58d5abb31129f8fae5de75220bc2176e77ff7c627ae45c918d9",
        strip_prefix = "crossbeam-utils-0.2.2",
        build_file = "//third_party/cargo/remote:crossbeam-utils-0.2.2.BUILD"
    )

    _new_http_archive(
        name = "raze__crossbeam_utils__0_5_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-utils/crossbeam-utils-0.5.0.crate",
        type = "tar.gz",
        sha256 = "677d453a17e8bd2b913fa38e8b9cf04bcdbb5be790aa294f2389661d72036015",
        strip_prefix = "crossbeam-utils-0.5.0",
        build_file = "//third_party/cargo/remote:crossbeam-utils-0.5.0.BUILD"
    )

    _new_http_archive(
        name = "raze__darling__0_6_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/darling/darling-0.6.3.crate",
        type = "tar.gz",
        sha256 = "49fc76d30c96cc0bdc8b966968e6535d900f3e42c56204d355192a670d989c6e",
        strip_prefix = "darling-0.6.3",
        build_file = "//third_party/cargo/remote:darling-0.6.3.BUILD"
    )

    _new_http_archive(
        name = "raze__darling_core__0_6_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/darling_core/darling_core-0.6.3.crate",
        type = "tar.gz",
        sha256 = "5d844ad185d7f9bfd072914584649741768151c4131f6ae59f282889f7a1e450",
        strip_prefix = "darling_core-0.6.3",
        build_file = "//third_party/cargo/remote:darling_core-0.6.3.BUILD"
    )

    _new_http_archive(
        name = "raze__darling_macro__0_6_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/darling_macro/darling_macro-0.6.3.crate",
        type = "tar.gz",
        sha256 = "280207f9bd6f6fd58acd08ed722fb9a75412ad9b1fd9b6a8fbfc55410aca2c2c",
        strip_prefix = "darling_macro-0.6.3",
        build_file = "//third_party/cargo/remote:darling_macro-0.6.3.BUILD"
    )

    _new_http_archive(
        name = "raze__deflate__0_7_18",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/deflate/deflate-0.7.18.crate",
        type = "tar.gz",
        sha256 = "32c8120d981901a9970a3a1c97cf8b630e0fa8c3ca31e75b6fd6fd5f9f427b31",
        strip_prefix = "deflate-0.7.18",
        build_file = "//third_party/cargo/remote:deflate-0.7.18.BUILD"
    )

    _new_http_archive(
        name = "raze__derivative__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/derivative/derivative-1.0.0.crate",
        type = "tar.gz",
        sha256 = "67b3d6d0e84e53a5bdc263cc59340541877bb541706a191d762bfac6a481bdde",
        strip_prefix = "derivative-1.0.0",
        build_file = "//third_party/cargo/remote:derivative-1.0.0.BUILD"
    )

    _new_http_archive(
        name = "raze__derive_new__0_5_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/derive-new/derive-new-0.5.5.crate",
        type = "tar.gz",
        sha256 = "899ec79626c14e00ccc9729b4d750bbe67fe76a8f436824c16e0233bbd9d7daa",
        strip_prefix = "derive-new-0.5.5",
        build_file = "//third_party/cargo/remote:derive-new-0.5.5.BUILD"
    )

    _new_http_archive(
        name = "raze__derive_builder__0_6_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/derive_builder/derive_builder-0.6.0.crate",
        type = "tar.gz",
        sha256 = "583a8f76cd41ae6303aca0db4539b90b4fcb289f75467d0c3905781dc670621b",
        strip_prefix = "derive_builder-0.6.0",
        build_file = "//third_party/cargo/remote:derive_builder-0.6.0.BUILD"
    )

    _new_http_archive(
        name = "raze__derive_builder_core__0_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/derive_builder_core/derive_builder_core-0.3.0.crate",
        type = "tar.gz",
        sha256 = "6fb4e6b5fb126caa298af7f9b9719ad6301eb7dd1613fd7543a4e935cef46c07",
        strip_prefix = "derive_builder_core-0.3.0",
        build_file = "//third_party/cargo/remote:derive_builder_core-0.3.0.BUILD"
    )

    _new_http_archive(
        name = "raze__diff__0_1_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/diff/diff-0.1.11.crate",
        type = "tar.gz",
        sha256 = "3c2b69f912779fbb121ceb775d74d51e915af17aaebc38d28a592843a2dd0a3a",
        strip_prefix = "diff-0.1.11",
        build_file = "//third_party/cargo/remote:diff-0.1.11.BUILD"
    )

    _new_http_archive(
        name = "raze__dylib__0_0_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/dylib/dylib-0.0.3.crate",
        type = "tar.gz",
        sha256 = "f06c13073013a912b363eee1433572499a2028a6b05432dad09383124d64731e",
        strip_prefix = "dylib-0.0.3",
        build_file = "//third_party/cargo/remote:dylib-0.0.3.BUILD"
    )

    _new_http_archive(
        name = "raze__either__1_5_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/either/either-1.5.0.crate",
        type = "tar.gz",
        sha256 = "3be565ca5c557d7f59e7cfcf1844f9e3033650c929c6566f511e8005f205c1d0",
        strip_prefix = "either-1.5.0",
        build_file = "//third_party/cargo/remote:either-1.5.0.BUILD"
    )

    _new_http_archive(
        name = "raze__ena__0_9_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ena/ena-0.9.3.crate",
        type = "tar.gz",
        sha256 = "88dc8393b3c7352f94092497f6b52019643e493b6b890eb417cdb7c46117e621",
        strip_prefix = "ena-0.9.3",
        build_file = "//third_party/cargo/remote:ena-0.9.3.BUILD"
    )

    _new_http_archive(
        name = "raze__encoding__0_2_33",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding/encoding-0.2.33.crate",
        type = "tar.gz",
        sha256 = "6b0d943856b990d12d3b55b359144ff341533e516d94098b1d3fc1ac666d36ec",
        strip_prefix = "encoding-0.2.33",
        build_file = "//third_party/cargo/remote:encoding-0.2.33.BUILD"
    )

    _new_http_archive(
        name = "raze__encoding_index_japanese__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-japanese/encoding-index-japanese-1.20141219.5.crate",
        type = "tar.gz",
        sha256 = "04e8b2ff42e9a05335dbf8b5c6f7567e5591d0d916ccef4e0b1710d32a0d0c91",
        strip_prefix = "encoding-index-japanese-1.20141219.5",
        build_file = "//third_party/cargo/remote:encoding-index-japanese-1.20141219.5.BUILD"
    )

    _new_http_archive(
        name = "raze__encoding_index_korean__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-korean/encoding-index-korean-1.20141219.5.crate",
        type = "tar.gz",
        sha256 = "4dc33fb8e6bcba213fe2f14275f0963fd16f0a02c878e3095ecfdf5bee529d81",
        strip_prefix = "encoding-index-korean-1.20141219.5",
        build_file = "//third_party/cargo/remote:encoding-index-korean-1.20141219.5.BUILD"
    )

    _new_http_archive(
        name = "raze__encoding_index_simpchinese__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-simpchinese/encoding-index-simpchinese-1.20141219.5.crate",
        type = "tar.gz",
        sha256 = "d87a7194909b9118fc707194baa434a4e3b0fb6a5a757c73c3adb07aa25031f7",
        strip_prefix = "encoding-index-simpchinese-1.20141219.5",
        build_file = "//third_party/cargo/remote:encoding-index-simpchinese-1.20141219.5.BUILD"
    )

    _new_http_archive(
        name = "raze__encoding_index_singlebyte__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-singlebyte/encoding-index-singlebyte-1.20141219.5.crate",
        type = "tar.gz",
        sha256 = "3351d5acffb224af9ca265f435b859c7c01537c0849754d3db3fdf2bfe2ae84a",
        strip_prefix = "encoding-index-singlebyte-1.20141219.5",
        build_file = "//third_party/cargo/remote:encoding-index-singlebyte-1.20141219.5.BUILD"
    )

    _new_http_archive(
        name = "raze__encoding_index_tradchinese__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-tradchinese/encoding-index-tradchinese-1.20141219.5.crate",
        type = "tar.gz",
        sha256 = "fd0e20d5688ce3cab59eb3ef3a2083a5c77bf496cb798dc6fcdb75f323890c18",
        strip_prefix = "encoding-index-tradchinese-1.20141219.5",
        build_file = "//third_party/cargo/remote:encoding-index-tradchinese-1.20141219.5.BUILD"
    )

    _new_http_archive(
        name = "raze__encoding_index_tests__0_1_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding_index_tests/encoding_index_tests-0.1.4.crate",
        type = "tar.gz",
        sha256 = "a246d82be1c9d791c5dfde9a2bd045fc3cbba3fa2b11ad558f27d01712f00569",
        strip_prefix = "encoding_index_tests-0.1.4",
        build_file = "//third_party/cargo/remote:encoding_index_tests-0.1.4.BUILD"
    )

    _new_http_archive(
        name = "raze__env_logger__0_5_13",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/env_logger/env_logger-0.5.13.crate",
        type = "tar.gz",
        sha256 = "15b0a4d2e39f8420210be8b27eeda28029729e2fd4291019455016c348240c38",
        strip_prefix = "env_logger-0.5.13",
        build_file = "//third_party/cargo/remote:env_logger-0.5.13.BUILD"
    )

    _new_http_archive(
        name = "raze__error_chain__0_12_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/error-chain/error-chain-0.12.0.crate",
        type = "tar.gz",
        sha256 = "07e791d3be96241c77c43846b665ef1384606da2cd2a48730abe606a12906e02",
        strip_prefix = "error-chain-0.12.0",
        build_file = "//third_party/cargo/remote:error-chain-0.12.0.BUILD"
    )

    _new_http_archive(
        name = "raze__failure__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/failure/failure-0.1.2.crate",
        type = "tar.gz",
        sha256 = "7efb22686e4a466b1ec1a15c2898f91fa9cb340452496dca654032de20ff95b9",
        strip_prefix = "failure-0.1.2",
        build_file = "//third_party/cargo/remote:failure-0.1.2.BUILD"
    )

    _new_http_archive(
        name = "raze__failure_derive__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/failure_derive/failure_derive-0.1.2.crate",
        type = "tar.gz",
        sha256 = "946d0e98a50d9831f5d589038d2ca7f8f455b1c21028c0db0e84116a12696426",
        strip_prefix = "failure_derive-0.1.2",
        build_file = "//third_party/cargo/remote:failure_derive-0.1.2.BUILD"
    )

    _new_http_archive(
        name = "raze__fern__0_5_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fern/fern-0.5.6.crate",
        type = "tar.gz",
        sha256 = "57915fe00a83af935983eb2d00b0ecc62419c4741b28c207ecbf98fd4a1b94c8",
        strip_prefix = "fern-0.5.6",
        build_file = "//third_party/cargo/remote:fern-0.5.6.BUILD"
    )

    _new_http_archive(
        name = "raze__fnv__1_0_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fnv/fnv-1.0.6.crate",
        type = "tar.gz",
        sha256 = "2fad85553e09a6f881f739c29f0b00b0f01357c743266d478b68951ce23285f3",
        strip_prefix = "fnv-1.0.6",
        build_file = "//third_party/cargo/remote:fnv-1.0.6.BUILD"
    )

    _new_http_archive(
        name = "raze__fuchsia_zircon__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon/fuchsia-zircon-0.3.3.crate",
        type = "tar.gz",
        sha256 = "2e9763c69ebaae630ba35f74888db465e49e259ba1bc0eda7d06f4a067615d82",
        strip_prefix = "fuchsia-zircon-0.3.3",
        build_file = "//third_party/cargo/remote:fuchsia-zircon-0.3.3.BUILD"
    )

    _new_http_archive(
        name = "raze__fuchsia_zircon_sys__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon-sys/fuchsia-zircon-sys-0.3.3.crate",
        type = "tar.gz",
        sha256 = "3dcaa9ae7725d12cdb85b3ad99a434db70b468c09ded17e012d86b5c1010f7a7",
        strip_prefix = "fuchsia-zircon-sys-0.3.3",
        build_file = "//third_party/cargo/remote:fuchsia-zircon-sys-0.3.3.BUILD"
    )

    _new_http_archive(
        name = "raze__futures__0_1_24",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/futures/futures-0.1.24.crate",
        type = "tar.gz",
        sha256 = "0c84b40c7e2de99ffd70602db314a7a8c26b2b3d830e6f7f7a142a8860ab3ca4",
        strip_prefix = "futures-0.1.24",
        build_file = "//third_party/cargo/remote:futures-0.1.24.BUILD"
    )

    _new_http_archive(
        name = "raze__futures_cpupool__0_1_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/futures-cpupool/futures-cpupool-0.1.8.crate",
        type = "tar.gz",
        sha256 = "ab90cde24b3319636588d0c35fe03b1333857621051837ed769faefb4c2162e4",
        strip_prefix = "futures-cpupool-0.1.8",
        build_file = "//third_party/cargo/remote:futures-cpupool-0.1.8.BUILD"
    )

    _new_http_archive(
        name = "raze__fxhash__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fxhash/fxhash-0.2.1.crate",
        type = "tar.gz",
        sha256 = "c31b6d751ae2c7f11320402d34e41349dd1016f8d5d45e48c4312bc8625af50c",
        strip_prefix = "fxhash-0.2.1",
        build_file = "//third_party/cargo/remote:fxhash-0.2.1.BUILD"
    )

    _new_http_archive(
        name = "raze__getopts__0_2_18",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/getopts/getopts-0.2.18.crate",
        type = "tar.gz",
        sha256 = "0a7292d30132fb5424b354f5dc02512a86e4c516fe544bb7a25e7f266951b797",
        strip_prefix = "getopts-0.2.18",
        build_file = "//third_party/cargo/remote:getopts-0.2.18.BUILD"
    )

    _new_http_archive(
        name = "raze__glob__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/glob/glob-0.2.11.crate",
        type = "tar.gz",
        sha256 = "8be18de09a56b60ed0edf84bc9df007e30040691af7acd1c41874faac5895bfb",
        strip_prefix = "glob-0.2.11",
        build_file = "//third_party/cargo/remote:glob-0.2.11.BUILD"
    )

    _new_http_archive(
        name = "raze__gnuplot__0_0_24",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/gnuplot/gnuplot-0.0.24.crate",
        type = "tar.gz",
        sha256 = "2cfbdbe219400e026809cf89a0d94af5f8e1736427e0a358272c47cd20f1e718",
        strip_prefix = "gnuplot-0.0.24",
        build_file = "//third_party/cargo/remote:gnuplot-0.0.24.BUILD"
    )

    _new_http_archive(
        name = "raze__h2__0_1_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/h2/h2-0.1.12.crate",
        type = "tar.gz",
        sha256 = "a27e7ed946e8335bdf9a191bc1b9b14a03ba822d013d2f58437f4fabcbd7fc2c",
        strip_prefix = "h2-0.1.12",
        build_file = "//third_party/cargo/remote:h2-0.1.12.BUILD"
    )

    _new_http_archive(
        name = "raze__heatmap__0_6_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/heatmap/heatmap-0.6.6.crate",
        type = "tar.gz",
        sha256 = "4c9551a9016b91c9b81fbc093e5ad0dd11c80ff4082fd2266170a210c2890051",
        strip_prefix = "heatmap-0.6.6",
        build_file = "//third_party/cargo/remote:heatmap-0.6.6.BUILD"
    )

    _new_http_archive(
        name = "raze__hibitset__0_5_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hibitset/hibitset-0.5.2.crate",
        type = "tar.gz",
        sha256 = "8875cbf0ea151f7e1267aba4482a9e0f8ef9440f3d2a57f4ca2363ae9b56070e",
        strip_prefix = "hibitset-0.5.2",
        build_file = "//third_party/cargo/remote:hibitset-0.5.2.BUILD"
    )

    _new_http_archive(
        name = "raze__histogram__0_6_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/histogram/histogram-0.6.9.crate",
        type = "tar.gz",
        sha256 = "12cb882ccb290b8646e554b157ab0b71e64e8d5bef775cd66b6531e52d302669",
        strip_prefix = "histogram-0.6.9",
        build_file = "//third_party/cargo/remote:histogram-0.6.9.BUILD"
    )

    _new_http_archive(
        name = "raze__hsl__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hsl/hsl-0.1.1.crate",
        type = "tar.gz",
        sha256 = "575fb7f1167f3b88ed825e90eb14918ac460461fdeaa3965c6a50951dee1c970",
        strip_prefix = "hsl-0.1.1",
        build_file = "//third_party/cargo/remote:hsl-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__http__0_1_13",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/http/http-0.1.13.crate",
        type = "tar.gz",
        sha256 = "24f58e8c2d8e886055c3ead7b28793e1455270b5fb39650984c224bc538ba581",
        strip_prefix = "http-0.1.13",
        build_file = "//third_party/cargo/remote:http-0.1.13.BUILD"
    )

    _new_http_archive(
        name = "raze__httparse__1_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/httparse/httparse-1.3.2.crate",
        type = "tar.gz",
        sha256 = "7b6288d7db100340ca12873fd4d08ad1b8f206a9457798dfb17c018a33fee540",
        strip_prefix = "httparse-1.3.2",
        build_file = "//third_party/cargo/remote:httparse-1.3.2.BUILD"
    )

    _new_http_archive(
        name = "raze__humantime__1_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/humantime/humantime-1.1.1.crate",
        type = "tar.gz",
        sha256 = "0484fda3e7007f2a4a0d9c3a703ca38c71c54c55602ce4660c419fd32e188c9e",
        strip_prefix = "humantime-1.1.1",
        build_file = "//third_party/cargo/remote:humantime-1.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__hyper__0_12_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hyper/hyper-0.12.10.crate",
        type = "tar.gz",
        sha256 = "529d00e4c998cced1a15ffd53bbe203917b39ed6071281c16184ab0014ca6ff3",
        strip_prefix = "hyper-0.12.10",
        build_file = "//third_party/cargo/remote:hyper-0.12.10.BUILD"
    )

    _new_http_archive(
        name = "raze__ident_case__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ident_case/ident_case-1.0.0.crate",
        type = "tar.gz",
        sha256 = "3c9826188e666f2ed92071d2dadef6edc430b11b158b5b2b3f4babbcc891eaaa",
        strip_prefix = "ident_case-1.0.0",
        build_file = "//third_party/cargo/remote:ident_case-1.0.0.BUILD"
    )

    _new_http_archive(
        name = "raze__indexmap__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/indexmap/indexmap-1.0.1.crate",
        type = "tar.gz",
        sha256 = "08173ba1e906efb6538785a8844dd496f5d34f0a2d88038e95195172fc667220",
        strip_prefix = "indexmap-1.0.1",
        build_file = "//third_party/cargo/remote:indexmap-1.0.1.BUILD"
    )

    _new_http_archive(
        name = "raze__inflate__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/inflate/inflate-0.2.0.crate",
        type = "tar.gz",
        sha256 = "d1238524675af3938a7c74980899535854b88ba07907bb1c944abe5b8fc437e5",
        strip_prefix = "inflate-0.2.0",
        build_file = "//third_party/cargo/remote:inflate-0.2.0.BUILD"
    )

    _new_http_archive(
        name = "raze__inflate__0_3_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/inflate/inflate-0.3.4.crate",
        type = "tar.gz",
        sha256 = "f5f9f47468e9a76a6452271efadc88fe865a82be91fe75e6c0c57b87ccea59d4",
        strip_prefix = "inflate-0.3.4",
        build_file = "//third_party/cargo/remote:inflate-0.3.4.BUILD"
    )

    _new_http_archive(
        name = "raze__iovec__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/iovec/iovec-0.1.2.crate",
        type = "tar.gz",
        sha256 = "dbe6e417e7d0975db6512b90796e8ce223145ac4e33c377e4a42882a0e88bb08",
        strip_prefix = "iovec-0.1.2",
        build_file = "//third_party/cargo/remote:iovec-0.1.2.BUILD"
    )

    _new_http_archive(
        name = "raze__isatty__0_1_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/isatty/isatty-0.1.9.crate",
        type = "tar.gz",
        sha256 = "e31a8281fc93ec9693494da65fbf28c0c2aa60a2eaec25dc58e2f31952e95edc",
        strip_prefix = "isatty-0.1.9",
        build_file = "//third_party/cargo/remote:isatty-0.1.9.BUILD"
    )

    _new_http_archive(
        name = "raze__itertools__0_5_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itertools/itertools-0.5.10.crate",
        type = "tar.gz",
        sha256 = "4833d6978da405305126af4ac88569b5d71ff758581ce5a987dbfa3755f694fc",
        strip_prefix = "itertools-0.5.10",
        build_file = "//third_party/cargo/remote:itertools-0.5.10.BUILD"
    )

    _new_http_archive(
        name = "raze__itertools__0_6_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itertools/itertools-0.6.5.crate",
        type = "tar.gz",
        sha256 = "d3f2be4da1690a039e9ae5fd575f706a63ad5a2120f161b1d653c9da3930dd21",
        strip_prefix = "itertools-0.6.5",
        build_file = "//third_party/cargo/remote:itertools-0.6.5.BUILD"
    )

    _new_http_archive(
        name = "raze__itertools__0_7_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itertools/itertools-0.7.8.crate",
        type = "tar.gz",
        sha256 = "f58856976b776fedd95533137617a02fb25719f40e7d9b01c7043cd65474f450",
        strip_prefix = "itertools-0.7.8",
        build_file = "//third_party/cargo/remote:itertools-0.7.8.BUILD"
    )

    _new_http_archive(
        name = "raze__itoa__0_4_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itoa/itoa-0.4.3.crate",
        type = "tar.gz",
        sha256 = "1306f3464951f30e30d12373d31c79fbd52d236e5e896fd92f96ec7babbbe60b",
        strip_prefix = "itoa-0.4.3",
        build_file = "//third_party/cargo/remote:itoa-0.4.3.BUILD"
    )

    _new_http_archive(
        name = "raze__kernel32_sys__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/kernel32-sys/kernel32-sys-0.2.2.crate",
        type = "tar.gz",
        sha256 = "7507624b29483431c0ba2d82aece8ca6cdba9382bff4ddd0f7490560c056098d",
        strip_prefix = "kernel32-sys-0.2.2",
        build_file = "//third_party/cargo/remote:kernel32-sys-0.2.2.BUILD"
    )

    _new_http_archive(
        name = "raze__lazy_static__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-0.2.11.crate",
        type = "tar.gz",
        sha256 = "76f033c7ad61445c5b347c7382dd1237847eb1bce590fe50365dcb33d546be73",
        strip_prefix = "lazy_static-0.2.11",
        build_file = "//third_party/cargo/remote:lazy_static-0.2.11.BUILD"
    )

    _new_http_archive(
        name = "raze__lazy_static__1_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-1.1.0.crate",
        type = "tar.gz",
        sha256 = "ca488b89a5657b0a2ecd45b95609b3e848cf1755da332a0da46e2b2b1cb371a7",
        strip_prefix = "lazy_static-1.1.0",
        build_file = "//third_party/cargo/remote:lazy_static-1.1.0.BUILD"
    )

    _new_http_archive(
        name = "raze__lazycell__1_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazycell/lazycell-1.1.0.crate",
        type = "tar.gz",
        sha256 = "e26d4c411b39f0afcf2ba6fe502be90e6c9b299c952dbd86124782520a13cffd",
        strip_prefix = "lazycell-1.1.0",
        build_file = "//third_party/cargo/remote:lazycell-1.1.0.BUILD"
    )

    _new_http_archive(
        name = "raze__libc__0_2_43",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.43.crate",
        type = "tar.gz",
        sha256 = "76e3a3ef172f1a0b9a9ff0dd1491ae5e6c948b94479a3021819ba7d860c8645d",
        strip_prefix = "libc-0.2.43",
        build_file = "//third_party/cargo/remote:libc-0.2.43.BUILD"
    )

    _new_http_archive(
        name = "raze__libloading__0_5_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libloading/libloading-0.5.0.crate",
        type = "tar.gz",
        sha256 = "9c3ad660d7cb8c5822cd83d10897b0f1f1526792737a179e73896152f85b88c2",
        strip_prefix = "libloading-0.5.0",
        build_file = "//third_party/cargo/remote:libloading-0.5.0.BUILD"
    )

    _new_http_archive(
        name = "raze__libsqlite3_sys__0_9_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libsqlite3-sys/libsqlite3-sys-0.9.3.crate",
        type = "tar.gz",
        sha256 = "d3711dfd91a1081d2458ad2d06ea30a8755256e74038be2ad927d94e1c955ca8",
        strip_prefix = "libsqlite3-sys-0.9.3",
        build_file = "//third_party/cargo/remote:libsqlite3-sys-0.9.3.BUILD"
    )

    _new_http_archive(
        name = "raze__linked_hash_map__0_4_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/linked-hash-map/linked-hash-map-0.4.2.crate",
        type = "tar.gz",
        sha256 = "7860ec297f7008ff7a1e3382d7f7e1dcd69efc94751a2284bafc3d013c2aa939",
        strip_prefix = "linked-hash-map-0.4.2",
        build_file = "//third_party/cargo/remote:linked-hash-map-0.4.2.BUILD"
    )

    _new_http_archive(
        name = "raze__lock_api__0_1_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lock_api/lock_api-0.1.3.crate",
        type = "tar.gz",
        sha256 = "949826a5ccf18c1b3a7c3d57692778d21768b79e46eb9dd07bfc4c2160036c54",
        strip_prefix = "lock_api-0.1.3",
        build_file = "//third_party/cargo/remote:lock_api-0.1.3.BUILD"
    )

    _new_http_archive(
        name = "raze__log__0_3_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.3.9.crate",
        type = "tar.gz",
        sha256 = "e19e8d5c34a3e0e2223db8e060f9e8264aeeb5c5fc64a4ee9965c062211c024b",
        strip_prefix = "log-0.3.9",
        build_file = "//third_party/cargo/remote:log-0.3.9.BUILD"
    )

    _new_http_archive(
        name = "raze__log__0_4_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.4.5.crate",
        type = "tar.gz",
        sha256 = "d4fcce5fa49cc693c312001daf1d13411c4a5283796bac1084299ea3e567113f",
        strip_prefix = "log-0.4.5",
        build_file = "//third_party/cargo/remote:log-0.4.5.BUILD"
    )

    _new_http_archive(
        name = "raze__lru_cache__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lru-cache/lru-cache-0.1.1.crate",
        type = "tar.gz",
        sha256 = "4d06ff7ff06f729ce5f4e227876cb88d10bc59cd4ae1e09fbb2bde15c850dc21",
        strip_prefix = "lru-cache-0.1.1",
        build_file = "//third_party/cargo/remote:lru-cache-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__matches__0_1_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/matches/matches-0.1.8.crate",
        type = "tar.gz",
        sha256 = "7ffc5c5338469d4d3ea17d269fa8ea3512ad247247c30bd2df69e68309ed0a08",
        strip_prefix = "matches-0.1.8",
        build_file = "//third_party/cargo/remote:matches-0.1.8.BUILD"
    )

    _new_http_archive(
        name = "raze__memchr__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-1.0.2.crate",
        type = "tar.gz",
        sha256 = "148fab2e51b4f1cfc66da2a7c32981d1d3c083a803978268bb11fe4b86925e7a",
        strip_prefix = "memchr-1.0.2",
        build_file = "//third_party/cargo/remote:memchr-1.0.2.BUILD"
    )

    _new_http_archive(
        name = "raze__memchr__2_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-2.0.2.crate",
        type = "tar.gz",
        sha256 = "a3b4142ab8738a78c51896f704f83c11df047ff1bda9a92a661aa6361552d93d",
        strip_prefix = "memchr-2.0.2",
        build_file = "//third_party/cargo/remote:memchr-2.0.2.BUILD"
    )

    _new_http_archive(
        name = "raze__memoffset__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memoffset/memoffset-0.2.1.crate",
        type = "tar.gz",
        sha256 = "0f9dc261e2b62d7a622bf416ea3c5245cdd5d9a7fcc428c0d06804dfce1775b3",
        strip_prefix = "memoffset-0.2.1",
        build_file = "//third_party/cargo/remote:memoffset-0.2.1.BUILD"
    )

    _new_http_archive(
        name = "raze__mio__0_6_16",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mio/mio-0.6.16.crate",
        type = "tar.gz",
        sha256 = "71646331f2619b1026cc302f87a2b8b648d5c6dd6937846a16cc8ce0f347f432",
        strip_prefix = "mio-0.6.16",
        build_file = "//third_party/cargo/remote:mio-0.6.16.BUILD"
    )

    _new_http_archive(
        name = "raze__mio_extras__2_0_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mio-extras/mio-extras-2.0.5.crate",
        type = "tar.gz",
        sha256 = "46e73a04c2fa6250b8d802134d56d554a9ec2922bf977777c805ea5def61ce40",
        strip_prefix = "mio-extras-2.0.5",
        build_file = "//third_party/cargo/remote:mio-extras-2.0.5.BUILD"
    )

    _new_http_archive(
        name = "raze__mio_uds__0_6_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mio-uds/mio-uds-0.6.7.crate",
        type = "tar.gz",
        sha256 = "966257a94e196b11bb43aca423754d87429960a768de9414f3691d6957abf125",
        strip_prefix = "mio-uds-0.6.7",
        build_file = "//third_party/cargo/remote:mio-uds-0.6.7.BUILD"
    )

    _new_http_archive(
        name = "raze__miow__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/miow/miow-0.2.1.crate",
        type = "tar.gz",
        sha256 = "8c1f2f3b1cf331de6896aabf6e9d55dca90356cc9960cca7eaaf408a355ae919",
        strip_prefix = "miow-0.2.1",
        build_file = "//third_party/cargo/remote:miow-0.2.1.BUILD"
    )

    _new_http_archive(
        name = "raze__mopa__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mopa/mopa-0.2.2.crate",
        type = "tar.gz",
        sha256 = "a785740271256c230f57462d3b83e52f998433a7062fc18f96d5999474a9f915",
        strip_prefix = "mopa-0.2.2",
        build_file = "//third_party/cargo/remote:mopa-0.2.2.BUILD"
    )

    _new_http_archive(
        name = "raze__mpmc__0_1_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mpmc/mpmc-0.1.5.crate",
        type = "tar.gz",
        sha256 = "cb947c698d784291c6b1d97269b0615beb966178537d4502ce90970507e1cf3b",
        strip_prefix = "mpmc-0.1.5",
        build_file = "//third_party/cargo/remote:mpmc-0.1.5.BUILD"
    )

    _new_http_archive(
        name = "raze__net2__0_2_33",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/net2/net2-0.2.33.crate",
        type = "tar.gz",
        sha256 = "42550d9fb7b6684a6d404d9fa7250c2eb2646df731d1c06afc06dcee9e1bcf88",
        strip_prefix = "net2-0.2.33",
        build_file = "//third_party/cargo/remote:net2-0.2.33.BUILD"
    )

    _new_http_archive(
        name = "raze__nodrop__0_1_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/nodrop/nodrop-0.1.12.crate",
        type = "tar.gz",
        sha256 = "9a2228dca57108069a5262f2ed8bd2e82496d2e074a06d1ccc7ce1687b6ae0a2",
        strip_prefix = "nodrop-0.1.12",
        build_file = "//third_party/cargo/remote:nodrop-0.1.12.BUILD"
    )

    _new_http_archive(
        name = "raze__nom__3_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/nom/nom-3.2.1.crate",
        type = "tar.gz",
        sha256 = "05aec50c70fd288702bcd93284a8444607f3292dbdf2a30de5ea5dcdbe72287b",
        strip_prefix = "nom-3.2.1",
        build_file = "//third_party/cargo/remote:nom-3.2.1.BUILD"
    )

    _new_http_archive(
        name = "raze__num__0_1_42",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num/num-0.1.42.crate",
        type = "tar.gz",
        sha256 = "4703ad64153382334aa8db57c637364c322d3372e097840c72000dabdcf6156e",
        strip_prefix = "num-0.1.42",
        build_file = "//third_party/cargo/remote:num-0.1.42.BUILD"
    )

    _new_http_archive(
        name = "raze__num_integer__0_1_39",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-integer/num-integer-0.1.39.crate",
        type = "tar.gz",
        sha256 = "e83d528d2677f0518c570baf2b7abdcf0cd2d248860b68507bdcb3e91d4c0cea",
        strip_prefix = "num-integer-0.1.39",
        build_file = "//third_party/cargo/remote:num-integer-0.1.39.BUILD"
    )

    _new_http_archive(
        name = "raze__num_iter__0_1_37",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-iter/num-iter-0.1.37.crate",
        type = "tar.gz",
        sha256 = "af3fdbbc3291a5464dc57b03860ec37ca6bf915ed6ee385e7c6c052c422b2124",
        strip_prefix = "num-iter-0.1.37",
        build_file = "//third_party/cargo/remote:num-iter-0.1.37.BUILD"
    )

    _new_http_archive(
        name = "raze__num_traits__0_1_43",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-traits/num-traits-0.1.43.crate",
        type = "tar.gz",
        sha256 = "92e5113e9fd4cc14ded8e499429f396a20f98c772a47cc8622a736e1ec843c31",
        strip_prefix = "num-traits-0.1.43",
        build_file = "//third_party/cargo/remote:num-traits-0.1.43.BUILD"
    )

    _new_http_archive(
        name = "raze__num_traits__0_2_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-traits/num-traits-0.2.6.crate",
        type = "tar.gz",
        sha256 = "0b3a5d7cc97d6d30d8b9bc8fa19bf45349ffe46241e8816f50f62f6d6aaabee1",
        strip_prefix = "num-traits-0.2.6",
        build_file = "//third_party/cargo/remote:num-traits-0.2.6.BUILD"
    )

    _new_http_archive(
        name = "raze__num_cpus__1_8_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num_cpus/num_cpus-1.8.0.crate",
        type = "tar.gz",
        sha256 = "c51a3322e4bca9d212ad9a158a02abc6934d005490c054a2778df73a70aa0a30",
        strip_prefix = "num_cpus-1.8.0",
        build_file = "//third_party/cargo/remote:num_cpus-1.8.0.BUILD"
    )

    _new_http_archive(
        name = "raze__odds__0_2_26",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/odds/odds-0.2.26.crate",
        type = "tar.gz",
        sha256 = "4eae0151b9dacf24fcc170d9995e511669a082856a91f958a2fe380bfab3fb22",
        strip_prefix = "odds-0.2.26",
        build_file = "//third_party/cargo/remote:odds-0.2.26.BUILD"
    )

    _new_http_archive(
        name = "raze__owning_ref__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/owning_ref/owning_ref-0.3.3.crate",
        type = "tar.gz",
        sha256 = "cdf84f41639e037b484f93433aa3897863b561ed65c6e59c7073d7c561710f37",
        strip_prefix = "owning_ref-0.3.3",
        build_file = "//third_party/cargo/remote:owning_ref-0.3.3.BUILD"
    )

    _new_http_archive(
        name = "raze__parking_lot__0_5_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/parking_lot/parking_lot-0.5.5.crate",
        type = "tar.gz",
        sha256 = "d4d05f1349491390b1730afba60bb20d55761bef489a954546b58b4b34e1e2ac",
        strip_prefix = "parking_lot-0.5.5",
        build_file = "//third_party/cargo/remote:parking_lot-0.5.5.BUILD"
    )

    _new_http_archive(
        name = "raze__parking_lot__0_6_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/parking_lot/parking_lot-0.6.4.crate",
        type = "tar.gz",
        sha256 = "f0802bff09003b291ba756dc7e79313e51cc31667e94afbe847def490424cde5",
        strip_prefix = "parking_lot-0.6.4",
        build_file = "//third_party/cargo/remote:parking_lot-0.6.4.BUILD"
    )

    _new_http_archive(
        name = "raze__parking_lot_core__0_2_14",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/parking_lot_core/parking_lot_core-0.2.14.crate",
        type = "tar.gz",
        sha256 = "4db1a8ccf734a7bce794cc19b3df06ed87ab2f3907036b693c68f56b4d4537fa",
        strip_prefix = "parking_lot_core-0.2.14",
        build_file = "//third_party/cargo/remote:parking_lot_core-0.2.14.BUILD"
    )

    _new_http_archive(
        name = "raze__parking_lot_core__0_3_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/parking_lot_core/parking_lot_core-0.3.1.crate",
        type = "tar.gz",
        sha256 = "ad7f7e6ebdc79edff6fdcb87a55b620174f7a989e3eb31b65231f4af57f00b8c",
        strip_prefix = "parking_lot_core-0.3.1",
        build_file = "//third_party/cargo/remote:parking_lot_core-0.3.1.BUILD"
    )

    _new_http_archive(
        name = "raze__peeking_take_while__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/peeking_take_while/peeking_take_while-0.1.2.crate",
        type = "tar.gz",
        sha256 = "19b17cddbe7ec3f8bc800887bab5e717348c95ea2ca0b1bf0837fb964dc67099",
        strip_prefix = "peeking_take_while-0.1.2",
        build_file = "//third_party/cargo/remote:peeking_take_while-0.1.2.BUILD"
    )

    _new_http_archive(
        name = "raze__pkg_config__0_3_14",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/pkg-config/pkg-config-0.3.14.crate",
        type = "tar.gz",
        sha256 = "676e8eb2b1b4c9043511a9b7bea0915320d7e502b0a079fb03f9635a5252b18c",
        strip_prefix = "pkg-config-0.3.14",
        build_file = "//third_party/cargo/remote:pkg-config-0.3.14.BUILD"
    )

    _new_http_archive(
        name = "raze__png__0_11_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/png/png-0.11.0.crate",
        type = "tar.gz",
        sha256 = "f0b0cabbbd20c2d7f06dbf015e06aad59b6ca3d9ed14848783e98af9aaf19925",
        strip_prefix = "png-0.11.0",
        build_file = "//third_party/cargo/remote:png-0.11.0.BUILD"
    )

    _new_http_archive(
        name = "raze__png__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/png/png-0.7.0.crate",
        type = "tar.gz",
        sha256 = "48f397b84083c2753ba53c7b56ad023edb94512b2885ffe227c66ff7edb61868",
        strip_prefix = "png-0.7.0",
        build_file = "//third_party/cargo/remote:png-0.7.0.BUILD"
    )

    _new_http_archive(
        name = "raze__proc_macro2__0_3_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/proc-macro2/proc-macro2-0.3.8.crate",
        type = "tar.gz",
        sha256 = "1b06e2f335f48d24442b35a19df506a835fb3547bc3c06ef27340da9acf5cae7",
        strip_prefix = "proc-macro2-0.3.8",
        build_file = "//third_party/cargo/remote:proc-macro2-0.3.8.BUILD"
    )

    _new_http_archive(
        name = "raze__proc_macro2__0_4_19",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/proc-macro2/proc-macro2-0.4.19.crate",
        type = "tar.gz",
        sha256 = "ffe022fb8c8bd254524b0b3305906c1921fa37a84a644e29079a9e62200c3901",
        strip_prefix = "proc-macro2-0.4.19",
        build_file = "//third_party/cargo/remote:proc-macro2-0.4.19.BUILD"
    )

    _new_http_archive(
        name = "raze__protobuf__1_7_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/protobuf/protobuf-1.7.4.crate",
        type = "tar.gz",
        sha256 = "52fbc45bf6709565e44ef31847eb7407b3c3c80af811ee884a04da071dcca12b",
        strip_prefix = "protobuf-1.7.4",
        build_file = "//third_party/cargo/remote:protobuf-1.7.4.BUILD"
    )

    _new_http_archive(
        name = "raze__quick_error__1_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quick-error/quick-error-1.2.2.crate",
        type = "tar.gz",
        sha256 = "9274b940887ce9addde99c4eee6b5c44cc494b182b97e73dc8ffdcb3397fd3f0",
        strip_prefix = "quick-error-1.2.2",
        build_file = "//third_party/cargo/remote:quick-error-1.2.2.BUILD"
    )

    _new_http_archive(
        name = "raze__quote__0_3_15",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-0.3.15.crate",
        type = "tar.gz",
        sha256 = "7a6e920b65c65f10b2ae65c831a81a073a89edd28c7cce89475bff467ab4167a",
        strip_prefix = "quote-0.3.15",
        build_file = "//third_party/cargo/remote:quote-0.3.15.BUILD"
    )

    _new_http_archive(
        name = "raze__quote__0_5_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-0.5.2.crate",
        type = "tar.gz",
        sha256 = "9949cfe66888ffe1d53e6ec9d9f3b70714083854be20fd5e271b232a017401e8",
        strip_prefix = "quote-0.5.2",
        build_file = "//third_party/cargo/remote:quote-0.5.2.BUILD"
    )

    _new_http_archive(
        name = "raze__quote__0_6_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-0.6.8.crate",
        type = "tar.gz",
        sha256 = "dd636425967c33af890042c483632d33fa7a18f19ad1d7ea72e8998c6ef8dea5",
        strip_prefix = "quote-0.6.8",
        build_file = "//third_party/cargo/remote:quote-0.6.8.BUILD"
    )

    _new_http_archive(
        name = "raze__rand__0_3_22",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand/rand-0.3.22.crate",
        type = "tar.gz",
        sha256 = "15a732abf9d20f0ad8eeb6f909bf6868722d9a06e1e50802b6a70351f40b4eb1",
        strip_prefix = "rand-0.3.22",
        build_file = "//third_party/cargo/remote:rand-0.3.22.BUILD"
    )

    _new_http_archive(
        name = "raze__rand__0_4_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand/rand-0.4.3.crate",
        type = "tar.gz",
        sha256 = "8356f47b32624fef5b3301c1be97e5944ecdd595409cc5da11d05f211db6cfbd",
        strip_prefix = "rand-0.4.3",
        build_file = "//third_party/cargo/remote:rand-0.4.3.BUILD"
    )

    _new_http_archive(
        name = "raze__rand__0_5_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand/rand-0.5.5.crate",
        type = "tar.gz",
        sha256 = "e464cd887e869cddcae8792a4ee31d23c7edd516700695608f5b98c67ee0131c",
        strip_prefix = "rand-0.5.5",
        build_file = "//third_party/cargo/remote:rand-0.5.5.BUILD"
    )

    _new_http_archive(
        name = "raze__rand_core__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand_core/rand_core-0.2.1.crate",
        type = "tar.gz",
        sha256 = "edecf0f94da5551fc9b492093e30b041a891657db7940ee221f9d2f66e82eef2",
        strip_prefix = "rand_core-0.2.1",
        build_file = "//third_party/cargo/remote:rand_core-0.2.1.BUILD"
    )

    _new_http_archive(
        name = "raze__rayon__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rayon/rayon-1.0.2.crate",
        type = "tar.gz",
        sha256 = "df7a791f788cb4c516f0e091301a29c2b71ef680db5e644a7d68835c8ae6dbfa",
        strip_prefix = "rayon-1.0.2",
        build_file = "//third_party/cargo/remote:rayon-1.0.2.BUILD"
    )

    _new_http_archive(
        name = "raze__rayon_core__1_4_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rayon-core/rayon-core-1.4.1.crate",
        type = "tar.gz",
        sha256 = "b055d1e92aba6877574d8fe604a63c8b5df60f60e5982bf7ccbb1338ea527356",
        strip_prefix = "rayon-core-1.4.1",
        build_file = "//third_party/cargo/remote:rayon-core-1.4.1.BUILD"
    )

    _new_http_archive(
        name = "raze__redox_syscall__0_1_40",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_syscall/redox_syscall-0.1.40.crate",
        type = "tar.gz",
        sha256 = "c214e91d3ecf43e9a4e41e578973adeb14b474f2bee858742d127af75a0112b1",
        strip_prefix = "redox_syscall-0.1.40",
        build_file = "//third_party/cargo/remote:redox_syscall-0.1.40.BUILD"
    )

    _new_http_archive(
        name = "raze__redox_termios__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_termios/redox_termios-0.1.1.crate",
        type = "tar.gz",
        sha256 = "7e891cfe48e9100a70a3b6eb652fef28920c117d366339687bd5576160db0f76",
        strip_prefix = "redox_termios-0.1.1",
        build_file = "//third_party/cargo/remote:redox_termios-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__regex__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-0.2.11.crate",
        type = "tar.gz",
        sha256 = "9329abc99e39129fcceabd24cf5d85b4671ef7c29c50e972bc5afe32438ec384",
        strip_prefix = "regex-0.2.11",
        build_file = "//third_party/cargo/remote:regex-0.2.11.BUILD"
    )

    _new_http_archive(
        name = "raze__regex__1_0_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-1.0.5.crate",
        type = "tar.gz",
        sha256 = "2069749032ea3ec200ca51e4a31df41759190a88edca0d2d86ee8bedf7073341",
        strip_prefix = "regex-1.0.5",
        build_file = "//third_party/cargo/remote:regex-1.0.5.BUILD"
    )

    _new_http_archive(
        name = "raze__regex_syntax__0_5_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.5.6.crate",
        type = "tar.gz",
        sha256 = "7d707a4fa2637f2dca2ef9fd02225ec7661fe01a53623c1e6515b6916511f7a7",
        strip_prefix = "regex-syntax-0.5.6",
        build_file = "//third_party/cargo/remote:regex-syntax-0.5.6.BUILD"
    )

    _new_http_archive(
        name = "raze__regex_syntax__0_6_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.6.2.crate",
        type = "tar.gz",
        sha256 = "747ba3b235651f6e2f67dfa8bcdcd073ddb7c243cb21c442fc12395dfcac212d",
        strip_prefix = "regex-syntax-0.6.2",
        build_file = "//third_party/cargo/remote:regex-syntax-0.6.2.BUILD"
    )

    _new_http_archive(
        name = "raze__rusqlite__0_14_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rusqlite/rusqlite-0.14.0.crate",
        type = "tar.gz",
        sha256 = "c9d9118f1ce84d8d0b67f9779936432fb42bb620cef2122409d786892cce9a3c",
        strip_prefix = "rusqlite-0.14.0",
        build_file = "//third_party/cargo/remote:rusqlite-0.14.0.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_ap_arena__237_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-arena/rustc-ap-arena-237.0.0.crate",
        type = "tar.gz",
        sha256 = "2d24c8b3c1437fad023cb9472381216a1d41d82dbb2d2e6c7858bd6f50317719",
        strip_prefix = "rustc-ap-arena-237.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-arena-237.0.0.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_ap_rustc_cratesio_shim__237_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-rustc_cratesio_shim/rustc-ap-rustc_cratesio_shim-237.0.0.crate",
        type = "tar.gz",
        sha256 = "9c5b02c76cd1ee4e9c97c8228701796d6b7431e8f100dea2d8af1d6c2c2bad56",
        strip_prefix = "rustc-ap-rustc_cratesio_shim-237.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-rustc_cratesio_shim-237.0.0.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_ap_rustc_data_structures__237_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-rustc_data_structures/rustc-ap-rustc_data_structures-237.0.0.crate",
        type = "tar.gz",
        sha256 = "4076388154497fb9a007e3badd78e415402a5594111cd6bc7ce1420dd1b1818b",
        strip_prefix = "rustc-ap-rustc_data_structures-237.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-rustc_data_structures-237.0.0.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_ap_rustc_errors__237_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-rustc_errors/rustc-ap-rustc_errors-237.0.0.crate",
        type = "tar.gz",
        sha256 = "c6c11e4789cbc276ceaa87d326c234b1a2d1e0fe6017b88a8a25903200060acb",
        strip_prefix = "rustc-ap-rustc_errors-237.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-rustc_errors-237.0.0.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_ap_rustc_target__237_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-rustc_target/rustc-ap-rustc_target-237.0.0.crate",
        type = "tar.gz",
        sha256 = "25f711bb152b9d7cdd69410cfe6d99aeb1409c959e0fdf3c8ca4d220e568aa52",
        strip_prefix = "rustc-ap-rustc_target-237.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-rustc_target-237.0.0.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_ap_serialize__237_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-serialize/rustc-ap-serialize-237.0.0.crate",
        type = "tar.gz",
        sha256 = "57638db658d4942d3f30a12566836f9a67a636ed8002c8cae1c9231214e39929",
        strip_prefix = "rustc-ap-serialize-237.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-serialize-237.0.0.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_ap_syntax__237_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-syntax/rustc-ap-syntax-237.0.0.crate",
        type = "tar.gz",
        sha256 = "d6dbcf07abf7a9957dce8d34353d55dfb4cd882153181f24349f4690facb58f0",
        strip_prefix = "rustc-ap-syntax-237.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-syntax-237.0.0.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_ap_syntax_pos__237_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-syntax_pos/rustc-ap-syntax_pos-237.0.0.crate",
        type = "tar.gz",
        sha256 = "0915cb5e166cabe588a129dec2d47357077e96fb1f9b57318fbe217eac4ce508",
        strip_prefix = "rustc-ap-syntax_pos-237.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-syntax_pos-237.0.0.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_demangle__0_1_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-demangle/rustc-demangle-0.1.9.crate",
        type = "tar.gz",
        sha256 = "bcfe5b13211b4d78e5c2cadfebd7769197d95c639c35a50057eb4c05de811395",
        strip_prefix = "rustc-demangle-0.1.9",
        build_file = "//third_party/cargo/remote:rustc-demangle-0.1.9.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_hash__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-hash/rustc-hash-1.0.1.crate",
        type = "tar.gz",
        sha256 = "7540fc8b0c49f096ee9c961cda096467dce8084bec6bdca2fc83895fd9b28cb8",
        strip_prefix = "rustc-hash-1.0.1",
        build_file = "//third_party/cargo/remote:rustc-hash-1.0.1.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_rayon__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-rayon/rustc-rayon-0.1.1.crate",
        type = "tar.gz",
        sha256 = "8c6d5a683c6ba4ed37959097e88d71c9e8e26659a3cb5be8b389078e7ad45306",
        strip_prefix = "rustc-rayon-0.1.1",
        build_file = "//third_party/cargo/remote:rustc-rayon-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_rayon_core__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-rayon-core/rustc-rayon-core-0.1.1.crate",
        type = "tar.gz",
        sha256 = "40f06724db71e18d68b3b946fdf890ca8c921d9edccc1404fdfdb537b0d12649",
        strip_prefix = "rustc-rayon-core-0.1.1",
        build_file = "//third_party/cargo/remote:rustc-rayon-core-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_serialize__0_3_24",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-serialize/rustc-serialize-0.3.24.crate",
        type = "tar.gz",
        sha256 = "dcf128d1287d2ea9d80910b5f1120d0b8eede3fbf1abe91c40d39ea7d51e6fda",
        strip_prefix = "rustc-serialize-0.3.24",
        build_file = "//third_party/cargo/remote:rustc-serialize-0.3.24.BUILD"
    )

    _new_http_archive(
        name = "raze__rustc_version__0_2_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc_version/rustc_version-0.2.3.crate",
        type = "tar.gz",
        sha256 = "138e3e0acb6c9fb258b19b67cb8abd63c00679d2851805ea151465464fe9030a",
        strip_prefix = "rustc_version-0.2.3",
        build_file = "//third_party/cargo/remote:rustc_version-0.2.3.BUILD"
    )

    _new_http_archive(
        name = "raze__rustfmt_nightly__0_99_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustfmt-nightly/rustfmt-nightly-0.99.4.crate",
        type = "tar.gz",
        sha256 = "df2124994af9da1062649cf5ac3738ae8c1bc292153e991b39a98904e342b7a7",
        strip_prefix = "rustfmt-nightly-0.99.4",
        build_file = "//third_party/cargo/remote:rustfmt-nightly-0.99.4.BUILD"
    )

    _new_http_archive(
        name = "raze__rusttype__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rusttype/rusttype-0.1.2.crate",
        type = "tar.gz",
        sha256 = "07b8848db3b5b5ba97020c6a756c0fdf2dbf2ad7c0d06aa4344a3f2f49c3fe17",
        strip_prefix = "rusttype-0.1.2",
        build_file = "//third_party/cargo/remote:rusttype-0.1.2.BUILD"
    )

    _new_http_archive(
        name = "raze__ryu__0_2_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ryu/ryu-0.2.6.crate",
        type = "tar.gz",
        sha256 = "7153dd96dade874ab973e098cb62fcdbb89a03682e46b144fd09550998d4a4a7",
        strip_prefix = "ryu-0.2.6",
        build_file = "//third_party/cargo/remote:ryu-0.2.6.BUILD"
    )

    _new_http_archive(
        name = "raze__scoped_tls__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/scoped-tls/scoped-tls-0.1.2.crate",
        type = "tar.gz",
        sha256 = "332ffa32bf586782a3efaeb58f127980944bbc8c4d6913a86107ac2a5ab24b28",
        strip_prefix = "scoped-tls-0.1.2",
        build_file = "//third_party/cargo/remote:scoped-tls-0.1.2.BUILD"
    )

    _new_http_archive(
        name = "raze__scopeguard__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/scopeguard/scopeguard-0.3.3.crate",
        type = "tar.gz",
        sha256 = "94258f53601af11e6a49f722422f6e3425c52b06245a5cf9bc09908b174f5e27",
        strip_prefix = "scopeguard-0.3.3",
        build_file = "//third_party/cargo/remote:scopeguard-0.3.3.BUILD"
    )

    _new_http_archive(
        name = "raze__sdl2__0_31_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/sdl2/sdl2-0.31.0.crate",
        type = "tar.gz",
        sha256 = "a74c2a98a354b20713b90cce70aef9e927e46110d1bc4ef728fd74e0d53eba60",
        strip_prefix = "sdl2-0.31.0",
        build_file = "//third_party/cargo/remote:sdl2-0.31.0.BUILD"
    )

    _new_http_archive(
        name = "raze__sdl2_sys__0_31_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/sdl2-sys/sdl2-sys-0.31.0.crate",
        type = "tar.gz",
        sha256 = "5c543ce8a6e33a30cb909612eeeb22e693848211a84558d5a00bb11e791b7ab7",
        strip_prefix = "sdl2-sys-0.31.0",
        build_file = "//third_party/cargo/remote:sdl2-sys-0.31.0.BUILD"
    )

    _new_http_archive(
        name = "raze__semver__0_9_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/semver/semver-0.9.0.crate",
        type = "tar.gz",
        sha256 = "1d7eb9ef2c18661902cc47e535f9bc51b78acd254da71d375c2f6720d9a40403",
        strip_prefix = "semver-0.9.0",
        build_file = "//third_party/cargo/remote:semver-0.9.0.BUILD"
    )

    _new_http_archive(
        name = "raze__semver_parser__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/semver-parser/semver-parser-0.7.0.crate",
        type = "tar.gz",
        sha256 = "388a1df253eca08550bef6c72392cfe7c30914bf41df5269b68cbd6ff8f570a3",
        strip_prefix = "semver-parser-0.7.0",
        build_file = "//third_party/cargo/remote:semver-parser-0.7.0.BUILD"
    )

    _new_http_archive(
        name = "raze__serde__1_0_79",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde/serde-1.0.79.crate",
        type = "tar.gz",
        sha256 = "84257ccd054dc351472528c8587b4de2dbf0dc0fe2e634030c1a90bfdacebaa9",
        strip_prefix = "serde-1.0.79",
        build_file = "//third_party/cargo/remote:serde-1.0.79.BUILD"
    )

    _new_http_archive(
        name = "raze__serde_derive__1_0_79",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde_derive/serde_derive-1.0.79.crate",
        type = "tar.gz",
        sha256 = "31569d901045afbff7a9479f793177fe9259819aff10ab4f89ef69bbc5f567fe",
        strip_prefix = "serde_derive-1.0.79",
        build_file = "//third_party/cargo/remote:serde_derive-1.0.79.BUILD"
    )

    _new_http_archive(
        name = "raze__serde_json__1_0_27",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde_json/serde_json-1.0.27.crate",
        type = "tar.gz",
        sha256 = "59790990c5115d16027f00913e2e66de23a51f70422e549d2ad68c8c5f268f1c",
        strip_prefix = "serde_json-1.0.27",
        build_file = "//third_party/cargo/remote:serde_json-1.0.27.BUILD"
    )

    _new_http_archive(
        name = "raze__shred__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/shred/shred-0.7.0.crate",
        type = "tar.gz",
        sha256 = "d94a47a63681350e0e358f8223045015454c59e34589b930bc721be22602edd1",
        strip_prefix = "shred-0.7.0",
        build_file = "//third_party/cargo/remote:shred-0.7.0.BUILD"
    )

    _new_http_archive(
        name = "raze__shred_derive__0_5_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/shred-derive/shred-derive-0.5.0.crate",
        type = "tar.gz",
        sha256 = "4b66c7ec6c50c6ef9909dd10faa24c8e571dfda5200786021b36b3fed77ac36c",
        strip_prefix = "shred-derive-0.5.0",
        build_file = "//third_party/cargo/remote:shred-derive-0.5.0.BUILD"
    )

    _new_http_archive(
        name = "raze__shrev__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/shrev/shrev-1.0.1.crate",
        type = "tar.gz",
        sha256 = "ec60ed6f60a4b3cdc2ceacf57215db3408fbd8990f66a38686a31558cd9da482",
        strip_prefix = "shrev-1.0.1",
        build_file = "//third_party/cargo/remote:shrev-1.0.1.BUILD"
    )

    _new_http_archive(
        name = "raze__slab__0_4_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/slab/slab-0.4.1.crate",
        type = "tar.gz",
        sha256 = "5f9776d6b986f77b35c6cf846c11ad986ff128fe0b2b63a3628e3755e8d3102d",
        strip_prefix = "slab-0.4.1",
        build_file = "//third_party/cargo/remote:slab-0.4.1.BUILD"
    )

    _new_http_archive(
        name = "raze__smallvec__0_6_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/smallvec/smallvec-0.6.5.crate",
        type = "tar.gz",
        sha256 = "153ffa32fd170e9944f7e0838edf824a754ec4c1fc64746fcc9fe1f8fa602e5d",
        strip_prefix = "smallvec-0.6.5",
        build_file = "//third_party/cargo/remote:smallvec-0.6.5.BUILD"
    )

    _new_http_archive(
        name = "raze__specs__0_12_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/specs/specs-0.12.2.crate",
        type = "tar.gz",
        sha256 = "026fe81d6d6c247c3274f4f475af14d7b1cb0c21c6f3097ba62c5fb72640592f",
        strip_prefix = "specs-0.12.2",
        build_file = "//third_party/cargo/remote:specs-0.12.2.BUILD"
    )

    _new_http_archive(
        name = "raze__stable_deref_trait__1_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/stable_deref_trait/stable_deref_trait-1.1.1.crate",
        type = "tar.gz",
        sha256 = "dba1a27d3efae4351c8051072d619e3ade2820635c3958d826bfea39d59b54c8",
        strip_prefix = "stable_deref_trait-1.1.1",
        build_file = "//third_party/cargo/remote:stable_deref_trait-1.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__stb_truetype__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/stb_truetype/stb_truetype-0.1.2.crate",
        type = "tar.gz",
        sha256 = "fcf3270840fc9de208d63e836eb3fdebb85379e7532f42f1b2cbd505fb6fda08",
        strip_prefix = "stb_truetype-0.1.2",
        build_file = "//third_party/cargo/remote:stb_truetype-0.1.2.BUILD"
    )

    _new_http_archive(
        name = "raze__string__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/string/string-0.1.1.crate",
        type = "tar.gz",
        sha256 = "00caf261d6f90f588f8450b8e1230fa0d5be49ee6140fdfbcb55335aff350970",
        strip_prefix = "string-0.1.1",
        build_file = "//third_party/cargo/remote:string-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__strsim__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/strsim/strsim-0.7.0.crate",
        type = "tar.gz",
        sha256 = "bb4f380125926a99e52bc279241539c018323fab05ad6368b56f93d9369ff550",
        strip_prefix = "strsim-0.7.0",
        build_file = "//third_party/cargo/remote:strsim-0.7.0.BUILD"
    )

    _new_http_archive(
        name = "raze__syn__0_10_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.10.8.crate",
        type = "tar.gz",
        sha256 = "58fd09df59565db3399efbba34ba8a2fec1307511ebd245d0061ff9d42691673",
        strip_prefix = "syn-0.10.8",
        build_file = "//third_party/cargo/remote:syn-0.10.8.BUILD"
    )

    _new_http_archive(
        name = "raze__syn__0_11_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.11.11.crate",
        type = "tar.gz",
        sha256 = "d3b891b9015c88c576343b9b3e41c2c11a51c219ef067b264bd9c8aa9b441dad",
        strip_prefix = "syn-0.11.11",
        build_file = "//third_party/cargo/remote:syn-0.11.11.BUILD"
    )

    _new_http_archive(
        name = "raze__syn__0_13_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.13.11.crate",
        type = "tar.gz",
        sha256 = "14f9bf6292f3a61d2c716723fdb789a41bbe104168e6f496dc6497e531ea1b9b",
        strip_prefix = "syn-0.13.11",
        build_file = "//third_party/cargo/remote:syn-0.13.11.BUILD"
    )

    _new_http_archive(
        name = "raze__syn__0_14_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.14.9.crate",
        type = "tar.gz",
        sha256 = "261ae9ecaa397c42b960649561949d69311f08eeaea86a65696e6e46517cf741",
        strip_prefix = "syn-0.14.9",
        build_file = "//third_party/cargo/remote:syn-0.14.9.BUILD"
    )

    _new_http_archive(
        name = "raze__syn__0_15_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.15.4.crate",
        type = "tar.gz",
        sha256 = "9056ebe7f2d6a38bc63171816fd1d3430da5a43896de21676dc5c0a4b8274a11",
        strip_prefix = "syn-0.15.4",
        build_file = "//third_party/cargo/remote:syn-0.15.4.BUILD"
    )

    _new_http_archive(
        name = "raze__synom__0_11_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/synom/synom-0.11.3.crate",
        type = "tar.gz",
        sha256 = "a393066ed9010ebaed60b9eafa373d4b1baac186dd7e008555b0f702b51945b6",
        strip_prefix = "synom-0.11.3",
        build_file = "//third_party/cargo/remote:synom-0.11.3.BUILD"
    )

    _new_http_archive(
        name = "raze__synstructure__0_9_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/synstructure/synstructure-0.9.0.crate",
        type = "tar.gz",
        sha256 = "85bb9b7550d063ea184027c9b8c20ac167cd36d3e06b3a40bceb9d746dc1a7b7",
        strip_prefix = "synstructure-0.9.0",
        build_file = "//third_party/cargo/remote:synstructure-0.9.0.BUILD"
    )

    _new_http_archive(
        name = "raze__term__0_5_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/term/term-0.5.1.crate",
        type = "tar.gz",
        sha256 = "5e6b677dd1e8214ea1ef4297f85dbcbed8e8cdddb561040cc998ca2551c37561",
        strip_prefix = "term-0.5.1",
        build_file = "//third_party/cargo/remote:term-0.5.1.BUILD"
    )

    _new_http_archive(
        name = "raze__termcolor__0_3_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termcolor/termcolor-0.3.6.crate",
        type = "tar.gz",
        sha256 = "adc4587ead41bf016f11af03e55a624c06568b5a19db4e90fde573d805074f83",
        strip_prefix = "termcolor-0.3.6",
        build_file = "//third_party/cargo/remote:termcolor-0.3.6.BUILD"
    )

    _new_http_archive(
        name = "raze__termcolor__1_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termcolor/termcolor-1.0.4.crate",
        type = "tar.gz",
        sha256 = "4096add70612622289f2fdcdbd5086dc81c1e2675e6ae58d6c4f62a16c6d7f2f",
        strip_prefix = "termcolor-1.0.4",
        build_file = "//third_party/cargo/remote:termcolor-1.0.4.BUILD"
    )

    _new_http_archive(
        name = "raze__termion__1_5_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termion/termion-1.5.1.crate",
        type = "tar.gz",
        sha256 = "689a3bdfaab439fd92bc87df5c4c78417d3cbe537487274e9b0b2dce76e92096",
        strip_prefix = "termion-1.5.1",
        build_file = "//third_party/cargo/remote:termion-1.5.1.BUILD"
    )

    _new_http_archive(
        name = "raze__textwrap__0_10_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/textwrap/textwrap-0.10.0.crate",
        type = "tar.gz",
        sha256 = "307686869c93e71f94da64286f9a9524c0f308a9e1c87a583de8e9c9039ad3f6",
        strip_prefix = "textwrap-0.10.0",
        build_file = "//third_party/cargo/remote:textwrap-0.10.0.BUILD"
    )

    _new_http_archive(
        name = "raze__thread_local__0_3_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-0.3.6.crate",
        type = "tar.gz",
        sha256 = "c6b53e329000edc2b34dbe8545fd20e55a333362d0a321909685a19bd28c3f1b",
        strip_prefix = "thread_local-0.3.6",
        build_file = "//third_party/cargo/remote:thread_local-0.3.6.BUILD"
    )

    _new_http_archive(
        name = "raze__tic__0_6_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tic/tic-0.6.0.crate",
        type = "tar.gz",
        sha256 = "532e144c72c8eaa3633eaf75317d9e8dd213fa7840da2dabda7eac5a9cfd8780",
        strip_prefix = "tic-0.6.0",
        build_file = "//third_party/cargo/remote:tic-0.6.0.BUILD"
    )

    _new_http_archive(
        name = "raze__time__0_1_40",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/time/time-0.1.40.crate",
        type = "tar.gz",
        sha256 = "d825be0eb33fda1a7e68012d51e9c7f451dc1a69391e7fdc197060bb8c56667b",
        strip_prefix = "time-0.1.40",
        build_file = "//third_party/cargo/remote:time-0.1.40.BUILD"
    )

    _new_http_archive(
        name = "raze__tiny_http__0_5_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tiny_http/tiny_http-0.5.9.crate",
        type = "tar.gz",
        sha256 = "2f4d55c9a213880d1f0c89ded183f209c6e45b912ca6c7df6f93c163773572e1",
        strip_prefix = "tiny_http-0.5.9",
        build_file = "//third_party/cargo/remote:tiny_http-0.5.9.BUILD"
    )

    _new_http_archive(
        name = "raze__tokio__0_1_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio/tokio-0.1.8.crate",
        type = "tar.gz",
        sha256 = "fbb6a6e9db2702097bfdfddcb09841211ad423b86c75b5ddaca1d62842ac492c",
        strip_prefix = "tokio-0.1.8",
        build_file = "//third_party/cargo/remote:tokio-0.1.8.BUILD"
    )

    _new_http_archive(
        name = "raze__tokio_codec__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-codec/tokio-codec-0.1.0.crate",
        type = "tar.gz",
        sha256 = "881e9645b81c2ce95fcb799ded2c29ffb9f25ef5bef909089a420e5961dd8ccb",
        strip_prefix = "tokio-codec-0.1.0",
        build_file = "//third_party/cargo/remote:tokio-codec-0.1.0.BUILD"
    )

    _new_http_archive(
        name = "raze__tokio_current_thread__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-current-thread/tokio-current-thread-0.1.1.crate",
        type = "tar.gz",
        sha256 = "8fdfb899688ac16f618076bd09215edbfda0fd5dfecb375b6942636cb31fa8a7",
        strip_prefix = "tokio-current-thread-0.1.1",
        build_file = "//third_party/cargo/remote:tokio-current-thread-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__tokio_executor__0_1_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-executor/tokio-executor-0.1.4.crate",
        type = "tar.gz",
        sha256 = "84823b932d566bc3c6aa644df4ca36cb38593c50b7db06011fd4e12e31e4047e",
        strip_prefix = "tokio-executor-0.1.4",
        build_file = "//third_party/cargo/remote:tokio-executor-0.1.4.BUILD"
    )

    _new_http_archive(
        name = "raze__tokio_fs__0_1_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-fs/tokio-fs-0.1.3.crate",
        type = "tar.gz",
        sha256 = "b5cbe4ca6e71cb0b62a66e4e6f53a8c06a6eefe46cc5f665ad6f274c9906f135",
        strip_prefix = "tokio-fs-0.1.3",
        build_file = "//third_party/cargo/remote:tokio-fs-0.1.3.BUILD"
    )

    _new_http_archive(
        name = "raze__tokio_io__0_1_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-io/tokio-io-0.1.8.crate",
        type = "tar.gz",
        sha256 = "8d6cc2de7725863c86ac71b0b9068476fec50834f055a243558ef1655bbd34cb",
        strip_prefix = "tokio-io-0.1.8",
        build_file = "//third_party/cargo/remote:tokio-io-0.1.8.BUILD"
    )

    _new_http_archive(
        name = "raze__tokio_reactor__0_1_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-reactor/tokio-reactor-0.1.5.crate",
        type = "tar.gz",
        sha256 = "4bfbaf9f260635649ec26b6fb4aded03887295ffcd999f6e43fd2c4758f758ea",
        strip_prefix = "tokio-reactor-0.1.5",
        build_file = "//third_party/cargo/remote:tokio-reactor-0.1.5.BUILD"
    )

    _new_http_archive(
        name = "raze__tokio_tcp__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-tcp/tokio-tcp-0.1.1.crate",
        type = "tar.gz",
        sha256 = "5b4c329b47f071eb8a746040465fa751bd95e4716e98daef6a9b4e434c17d565",
        strip_prefix = "tokio-tcp-0.1.1",
        build_file = "//third_party/cargo/remote:tokio-tcp-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__tokio_threadpool__0_1_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-threadpool/tokio-threadpool-0.1.6.crate",
        type = "tar.gz",
        sha256 = "a5758cecb6e0633cea5d563ac07c975e04961690b946b04fd84e7d6445a8f6af",
        strip_prefix = "tokio-threadpool-0.1.6",
        build_file = "//third_party/cargo/remote:tokio-threadpool-0.1.6.BUILD"
    )

    _new_http_archive(
        name = "raze__tokio_timer__0_2_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-timer/tokio-timer-0.2.6.crate",
        type = "tar.gz",
        sha256 = "d03fa701f9578a01b7014f106b47f0a363b4727a7f3f75d666e312ab7acbbf1c",
        strip_prefix = "tokio-timer-0.2.6",
        build_file = "//third_party/cargo/remote:tokio-timer-0.2.6.BUILD"
    )

    _new_http_archive(
        name = "raze__tokio_udp__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-udp/tokio-udp-0.1.2.crate",
        type = "tar.gz",
        sha256 = "da941144b816d0dcda4db3a1ba87596e4df5e860a72b70783fe435891f80601c",
        strip_prefix = "tokio-udp-0.1.2",
        build_file = "//third_party/cargo/remote:tokio-udp-0.1.2.BUILD"
    )

    _new_http_archive(
        name = "raze__tokio_uds__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-uds/tokio-uds-0.2.1.crate",
        type = "tar.gz",
        sha256 = "424c1ed15a0132251813ccea50640b224c809d6ceafb88154c1a8775873a0e89",
        strip_prefix = "tokio-uds-0.2.1",
        build_file = "//third_party/cargo/remote:tokio-uds-0.2.1.BUILD"
    )

    _new_http_archive(
        name = "raze__toml__0_4_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/toml/toml-0.4.6.crate",
        type = "tar.gz",
        sha256 = "a0263c6c02c4db6c8f7681f9fd35e90de799ebd4cfdeab77a38f4ff6b3d8c0d9",
        strip_prefix = "toml-0.4.6",
        build_file = "//third_party/cargo/remote:toml-0.4.6.BUILD"
    )

    _new_http_archive(
        name = "raze__try_lock__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/try-lock/try-lock-0.2.2.crate",
        type = "tar.gz",
        sha256 = "e604eb7b43c06650e854be16a2a03155743d3752dd1c943f6829e26b7a36e382",
        strip_prefix = "try-lock-0.2.2",
        build_file = "//third_party/cargo/remote:try-lock-0.2.2.BUILD"
    )

    _new_http_archive(
        name = "raze__tuple_utils__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tuple_utils/tuple_utils-0.2.0.crate",
        type = "tar.gz",
        sha256 = "cbfecd7bb8f0a3e96b3b31c46af2677a55a588767c0091f484601424fcb20e7e",
        strip_prefix = "tuple_utils-0.2.0",
        build_file = "//third_party/cargo/remote:tuple_utils-0.2.0.BUILD"
    )

    _new_http_archive(
        name = "raze__ucd_util__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ucd-util/ucd-util-0.1.1.crate",
        type = "tar.gz",
        sha256 = "fd2be2d6639d0f8fe6cdda291ad456e23629558d466e2789d2c3e9892bda285d",
        strip_prefix = "ucd-util-0.1.1",
        build_file = "//third_party/cargo/remote:ucd-util-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__unicode_segmentation__1_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-segmentation/unicode-segmentation-1.2.1.crate",
        type = "tar.gz",
        sha256 = "aa6024fc12ddfd1c6dbc14a80fa2324d4568849869b779f6bd37e5e4c03344d1",
        strip_prefix = "unicode-segmentation-1.2.1",
        build_file = "//third_party/cargo/remote:unicode-segmentation-1.2.1.BUILD"
    )

    _new_http_archive(
        name = "raze__unicode_width__0_1_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-width/unicode-width-0.1.5.crate",
        type = "tar.gz",
        sha256 = "882386231c45df4700b275c7ff55b6f3698780a650026380e72dabe76fa46526",
        strip_prefix = "unicode-width-0.1.5",
        build_file = "//third_party/cargo/remote:unicode-width-0.1.5.BUILD"
    )

    _new_http_archive(
        name = "raze__unicode_xid__0_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-xid/unicode-xid-0.0.4.crate",
        type = "tar.gz",
        sha256 = "8c1f860d7d29cf02cb2f3f359fd35991af3d30bac52c57d265a3c461074cb4dc",
        strip_prefix = "unicode-xid-0.0.4",
        build_file = "//third_party/cargo/remote:unicode-xid-0.0.4.BUILD"
    )

    _new_http_archive(
        name = "raze__unicode_xid__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-xid/unicode-xid-0.1.0.crate",
        type = "tar.gz",
        sha256 = "fc72304796d0818e357ead4e000d19c9c174ab23dc11093ac919054d20a6a7fc",
        strip_prefix = "unicode-xid-0.1.0",
        build_file = "//third_party/cargo/remote:unicode-xid-0.1.0.BUILD"
    )

    _new_http_archive(
        name = "raze__unreachable__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unreachable/unreachable-1.0.0.crate",
        type = "tar.gz",
        sha256 = "382810877fe448991dfc7f0dd6e3ae5d58088fd0ea5e35189655f84e6814fa56",
        strip_prefix = "unreachable-1.0.0",
        build_file = "//third_party/cargo/remote:unreachable-1.0.0.BUILD"
    )

    _new_http_archive(
        name = "raze__url__0_2_38",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/url/url-0.2.38.crate",
        type = "tar.gz",
        sha256 = "cbaa8377a162d88e7d15db0cf110c8523453edcbc5bc66d2b6fffccffa34a068",
        strip_prefix = "url-0.2.38",
        build_file = "//third_party/cargo/remote:url-0.2.38.BUILD"
    )

    _new_http_archive(
        name = "raze__utf8_ranges__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/utf8-ranges/utf8-ranges-1.0.1.crate",
        type = "tar.gz",
        sha256 = "fd70f467df6810094968e2fce0ee1bd0e87157aceb026a8c083bcf5e25b9efe4",
        strip_prefix = "utf8-ranges-1.0.1",
        build_file = "//third_party/cargo/remote:utf8-ranges-1.0.1.BUILD"
    )

    _new_http_archive(
        name = "raze__uuid__0_1_18",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/uuid/uuid-0.1.18.crate",
        type = "tar.gz",
        sha256 = "78c590b5bd79ed10aad8fb75f078a59d8db445af6c743e55c4a53227fc01c13f",
        strip_prefix = "uuid-0.1.18",
        build_file = "//third_party/cargo/remote:uuid-0.1.18.BUILD"
    )

    _new_http_archive(
        name = "raze__vcpkg__0_2_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/vcpkg/vcpkg-0.2.6.crate",
        type = "tar.gz",
        sha256 = "def296d3eb3b12371b2c7d0e83bfe1403e4db2d7a0bba324a12b21c4ee13143d",
        strip_prefix = "vcpkg-0.2.6",
        build_file = "//third_party/cargo/remote:vcpkg-0.2.6.BUILD"
    )

    _new_http_archive(
        name = "raze__vec_map__0_8_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/vec_map/vec_map-0.8.1.crate",
        type = "tar.gz",
        sha256 = "05c78687fb1a80548ae3250346c3db86a80a7cdd77bda190189f2d0a0987c81a",
        strip_prefix = "vec_map-0.8.1",
        build_file = "//third_party/cargo/remote:vec_map-0.8.1.BUILD"
    )

    _new_http_archive(
        name = "raze__version_check__0_1_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/version_check/version_check-0.1.4.crate",
        type = "tar.gz",
        sha256 = "7716c242968ee87e5542f8021178248f267f295a5c4803beae8b8b7fd9bc6051",
        strip_prefix = "version_check-0.1.4",
        build_file = "//third_party/cargo/remote:version_check-0.1.4.BUILD"
    )

    _new_http_archive(
        name = "raze__vk_sys__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/vk-sys/vk-sys-0.3.3.crate",
        type = "tar.gz",
        sha256 = "9567ee6b79b72dfe50201817a9b903de91a1deb091b41c165c2c3679884d8103",
        strip_prefix = "vk-sys-0.3.3",
        build_file = "//third_party/cargo/remote:vk-sys-0.3.3.BUILD"
    )

    _new_http_archive(
        name = "raze__void__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/void/void-1.0.2.crate",
        type = "tar.gz",
        sha256 = "6a02e4885ed3bc0f2de90ea6dd45ebcbb66dacffe03547fadbb0eeae2770887d",
        strip_prefix = "void-1.0.2",
        build_file = "//third_party/cargo/remote:void-1.0.2.BUILD"
    )

    _new_http_archive(
        name = "raze__want__0_0_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/want/want-0.0.6.crate",
        type = "tar.gz",
        sha256 = "797464475f30ddb8830cc529aaaae648d581f99e2036a928877dfde027ddf6b3",
        strip_prefix = "want-0.0.6",
        build_file = "//third_party/cargo/remote:want-0.0.6.BUILD"
    )

    _new_http_archive(
        name = "raze__waterfall__0_7_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/waterfall/waterfall-0.7.1.crate",
        type = "tar.gz",
        sha256 = "ddfd2a19feb20d152820c6d01acfda726c305fa7ea67f685359d24f4d6040729",
        strip_prefix = "waterfall-0.7.1",
        build_file = "//third_party/cargo/remote:waterfall-0.7.1.BUILD"
    )

    _new_http_archive(
        name = "raze__which__1_0_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/which/which-1.0.5.crate",
        type = "tar.gz",
        sha256 = "e84a603e7e0b1ce1aa1ee2b109c7be00155ce52df5081590d1ffb93f4f515cb2",
        strip_prefix = "which-1.0.5",
        build_file = "//third_party/cargo/remote:which-1.0.5.BUILD"
    )

    _new_http_archive(
        name = "raze__winapi__0_2_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.2.8.crate",
        type = "tar.gz",
        sha256 = "167dc9d6949a9b857f3451275e911c3f44255842c1f7a76f33c55103a909087a",
        strip_prefix = "winapi-0.2.8",
        build_file = "//third_party/cargo/remote:winapi-0.2.8.BUILD"
    )

    _new_http_archive(
        name = "raze__winapi__0_3_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.5.crate",
        type = "tar.gz",
        sha256 = "773ef9dcc5f24b7d850d0ff101e542ff24c3b090a9768e03ff889fdef41f00fd",
        strip_prefix = "winapi-0.3.5",
        build_file = "//third_party/cargo/remote:winapi-0.3.5.BUILD"
    )

    _new_http_archive(
        name = "raze__winapi_build__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-build/winapi-build-0.1.1.crate",
        type = "tar.gz",
        sha256 = "2d315eee3b34aca4797b2da6b13ed88266e6d612562a0c46390af8299fc699bc",
        strip_prefix = "winapi-build-0.1.1",
        build_file = "//third_party/cargo/remote:winapi-build-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = "//third_party/cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD"
    )

    _new_http_archive(
        name = "raze__winapi_util__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-util/winapi-util-0.1.1.crate",
        type = "tar.gz",
        sha256 = "afc5508759c5bf4285e61feb862b6083c8480aec864fa17a81fdec6f69b461ab",
        strip_prefix = "winapi-util-0.1.1",
        build_file = "//third_party/cargo/remote:winapi-util-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = "//third_party/cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD"
    )

    _new_http_archive(
        name = "raze__wincolor__0_1_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/wincolor/wincolor-0.1.6.crate",
        type = "tar.gz",
        sha256 = "eeb06499a3a4d44302791052df005d5232b927ed1a9658146d842165c4de7767",
        strip_prefix = "wincolor-0.1.6",
        build_file = "//third_party/cargo/remote:wincolor-0.1.6.BUILD"
    )

    _new_http_archive(
        name = "raze__wincolor__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/wincolor/wincolor-1.0.1.crate",
        type = "tar.gz",
        sha256 = "561ed901ae465d6185fa7864d63fbd5720d0ef718366c9a4dc83cf6170d7e9ba",
        strip_prefix = "wincolor-1.0.1",
        build_file = "//third_party/cargo/remote:wincolor-1.0.1.BUILD"
    )

    _new_http_archive(
        name = "raze__ws2_32_sys__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ws2_32-sys/ws2_32-sys-0.2.1.crate",
        type = "tar.gz",
        sha256 = "d59cefebd0c892fa2dd6de581e937301d8552cb44489cdff035c6187cb63fa5e",
        strip_prefix = "ws2_32-sys-0.2.1",
        build_file = "//third_party/cargo/remote:ws2_32-sys-0.2.1.BUILD"
    )

    _new_http_archive(
        name = "raze__x11_dl__2_18_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/x11-dl/x11-dl-2.18.3.crate",
        type = "tar.gz",
        sha256 = "940586acb859ea05c53971ac231685799a7ec1dee66ac0bccc0e6ad96e06b4e3",
        strip_prefix = "x11-dl-2.18.3",
        build_file = "//third_party/cargo/remote:x11-dl-2.18.3.BUILD"
    )

    _new_http_archive(
        name = "raze__zcfg__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/zcfg/zcfg-0.2.0.crate",
        type = "tar.gz",
        sha256 = "9aa41d4f5c82f620718be78e594b55c82f8e5ba279f46dff436f47b5326b765d",
        strip_prefix = "zcfg-0.2.0",
        build_file = "//third_party/cargo/remote:zcfg-0.2.0.BUILD"
    )

    _new_http_archive(
        name = "raze__zcfg_flag_parser__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/zcfg_flag_parser/zcfg_flag_parser-0.2.0.crate",
        type = "tar.gz",
        sha256 = "7185c4c58c157b54f917d8ffa10037b234ca9dfbb4a6e6741ae0cd951e395d32",
        strip_prefix = "zcfg_flag_parser-0.2.0",
        build_file = "//third_party/cargo/remote:zcfg_flag_parser-0.2.0.BUILD"
    )

