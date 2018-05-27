"""
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""

def raze_fetch_remote_crates():

    native.new_http_archive(
        name = "raze__adler32__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/adler32/adler32-1.0.2.crate",
        type = "tar.gz",
        strip_prefix = "adler32-1.0.2",
        build_file = "//third_party/cargo/remote:adler32-1.0.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__aho_corasick__0_6_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.6.4.crate",
        type = "tar.gz",
        strip_prefix = "aho-corasick-0.6.4",
        build_file = "//third_party/cargo/remote:aho-corasick-0.6.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__allan__0_2_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/allan/allan-0.2.4.crate",
        type = "tar.gz",
        strip_prefix = "allan-0.2.4",
        build_file = "//third_party/cargo/remote:allan-0.2.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__ansi_term__0_11_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ansi_term/ansi_term-0.11.0.crate",
        type = "tar.gz",
        strip_prefix = "ansi_term-0.11.0",
        build_file = "//third_party/cargo/remote:ansi_term-0.11.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__approx__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/approx/approx-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "approx-0.1.1",
        build_file = "//third_party/cargo/remote:approx-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__arrayvec__0_3_25",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/arrayvec/arrayvec-0.3.25.crate",
        type = "tar.gz",
        strip_prefix = "arrayvec-0.3.25",
        build_file = "//third_party/cargo/remote:arrayvec-0.3.25.BUILD"
    )

    native.new_http_archive(
        name = "raze__arrayvec__0_4_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/arrayvec/arrayvec-0.4.7.crate",
        type = "tar.gz",
        strip_prefix = "arrayvec-0.4.7",
        build_file = "//third_party/cargo/remote:arrayvec-0.4.7.BUILD"
    )

    native.new_http_archive(
        name = "raze__ascii__0_7_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ascii/ascii-0.7.1.crate",
        type = "tar.gz",
        strip_prefix = "ascii-0.7.1",
        build_file = "//third_party/cargo/remote:ascii-0.7.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__atom__0_3_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atom/atom-0.3.5.crate",
        type = "tar.gz",
        strip_prefix = "atom-0.3.5",
        build_file = "//third_party/cargo/remote:atom-0.3.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__atty__0_2_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atty/atty-0.2.10.crate",
        type = "tar.gz",
        strip_prefix = "atty-0.2.10",
        build_file = "//third_party/cargo/remote:atty-0.2.10.BUILD"
    )

    native.new_http_archive(
        name = "raze__backtrace__0_3_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/backtrace/backtrace-0.3.8.crate",
        type = "tar.gz",
        strip_prefix = "backtrace-0.3.8",
        build_file = "//third_party/cargo/remote:backtrace-0.3.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__backtrace_sys__0_1_21",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/backtrace-sys/backtrace-sys-0.1.21.crate",
        type = "tar.gz",
        strip_prefix = "backtrace-sys-0.1.21",
        build_file = "//third_party/cargo/remote:backtrace-sys-0.1.21.BUILD"
    )

    native.new_http_archive(
        name = "raze__base64__0_9_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/base64/base64-0.9.1.crate",
        type = "tar.gz",
        strip_prefix = "base64-0.9.1",
        build_file = "//third_party/cargo/remote:base64-0.9.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__bindgen__0_35_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bindgen/bindgen-0.35.0.crate",
        type = "tar.gz",
        strip_prefix = "bindgen-0.35.0",
        build_file = "//third_party/cargo/remote:bindgen-0.35.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__bitflags__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-0.7.0.crate",
        type = "tar.gz",
        strip_prefix = "bitflags-0.7.0",
        build_file = "//third_party/cargo/remote:bitflags-0.7.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__bitflags__1_0_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-1.0.3.crate",
        type = "tar.gz",
        strip_prefix = "bitflags-1.0.3",
        build_file = "//third_party/cargo/remote:bitflags-1.0.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__byteorder__0_4_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/byteorder/byteorder-0.4.2.crate",
        type = "tar.gz",
        strip_prefix = "byteorder-0.4.2",
        build_file = "//third_party/cargo/remote:byteorder-0.4.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__byteorder__1_2_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/byteorder/byteorder-1.2.3.crate",
        type = "tar.gz",
        strip_prefix = "byteorder-1.2.3",
        build_file = "//third_party/cargo/remote:byteorder-1.2.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__bytes__0_4_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bytes/bytes-0.4.8.crate",
        type = "tar.gz",
        strip_prefix = "bytes-0.4.8",
        build_file = "//third_party/cargo/remote:bytes-0.4.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__cargo_metadata__0_4_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cargo_metadata/cargo_metadata-0.4.1.crate",
        type = "tar.gz",
        strip_prefix = "cargo_metadata-0.4.1",
        build_file = "//third_party/cargo/remote:cargo_metadata-0.4.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__cc__1_0_15",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cc/cc-1.0.15.crate",
        type = "tar.gz",
        strip_prefix = "cc-1.0.15",
        build_file = "//third_party/cargo/remote:cc-1.0.15.BUILD"
    )

    native.new_http_archive(
        name = "raze__cexpr__0_2_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cexpr/cexpr-0.2.3.crate",
        type = "tar.gz",
        strip_prefix = "cexpr-0.2.3",
        build_file = "//third_party/cargo/remote:cexpr-0.2.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__cfg_if__0_1_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.3.crate",
        type = "tar.gz",
        strip_prefix = "cfg-if-0.1.3",
        build_file = "//third_party/cargo/remote:cfg-if-0.1.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__cgmath__0_16_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cgmath/cgmath-0.16.1.crate",
        type = "tar.gz",
        strip_prefix = "cgmath-0.16.1",
        build_file = "//third_party/cargo/remote:cgmath-0.16.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__chrono__0_2_25",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/chrono/chrono-0.2.25.crate",
        type = "tar.gz",
        strip_prefix = "chrono-0.2.25",
        build_file = "//third_party/cargo/remote:chrono-0.2.25.BUILD"
    )

    native.new_http_archive(
        name = "raze__chrono__0_4_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/chrono/chrono-0.4.2.crate",
        type = "tar.gz",
        strip_prefix = "chrono-0.4.2",
        build_file = "//third_party/cargo/remote:chrono-0.4.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__chunked_transfer__0_3_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/chunked_transfer/chunked_transfer-0.3.1.crate",
        type = "tar.gz",
        strip_prefix = "chunked_transfer-0.3.1",
        build_file = "//third_party/cargo/remote:chunked_transfer-0.3.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__clang_sys__0_22_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clang-sys/clang-sys-0.22.0.crate",
        type = "tar.gz",
        strip_prefix = "clang-sys-0.22.0",
        build_file = "//third_party/cargo/remote:clang-sys-0.22.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__clap__2_31_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clap/clap-2.31.2.crate",
        type = "tar.gz",
        strip_prefix = "clap-2.31.2",
        build_file = "//third_party/cargo/remote:clap-2.31.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__clocksource__0_2_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clocksource/clocksource-0.2.4.crate",
        type = "tar.gz",
        strip_prefix = "clocksource-0.2.4",
        build_file = "//third_party/cargo/remote:clocksource-0.2.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__crossbeam__0_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam/crossbeam-0.3.2.crate",
        type = "tar.gz",
        strip_prefix = "crossbeam-0.3.2",
        build_file = "//third_party/cargo/remote:crossbeam-0.3.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__crossbeam_deque__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-deque/crossbeam-deque-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "crossbeam-deque-0.2.0",
        build_file = "//third_party/cargo/remote:crossbeam-deque-0.2.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__crossbeam_deque__0_3_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-deque/crossbeam-deque-0.3.1.crate",
        type = "tar.gz",
        strip_prefix = "crossbeam-deque-0.3.1",
        build_file = "//third_party/cargo/remote:crossbeam-deque-0.3.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__crossbeam_epoch__0_3_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-epoch/crossbeam-epoch-0.3.1.crate",
        type = "tar.gz",
        strip_prefix = "crossbeam-epoch-0.3.1",
        build_file = "//third_party/cargo/remote:crossbeam-epoch-0.3.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__crossbeam_epoch__0_4_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-epoch/crossbeam-epoch-0.4.1.crate",
        type = "tar.gz",
        strip_prefix = "crossbeam-epoch-0.4.1",
        build_file = "//third_party/cargo/remote:crossbeam-epoch-0.4.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__crossbeam_utils__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-utils/crossbeam-utils-0.2.2.crate",
        type = "tar.gz",
        strip_prefix = "crossbeam-utils-0.2.2",
        build_file = "//third_party/cargo/remote:crossbeam-utils-0.2.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__crossbeam_utils__0_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-utils/crossbeam-utils-0.3.2.crate",
        type = "tar.gz",
        strip_prefix = "crossbeam-utils-0.3.2",
        build_file = "//third_party/cargo/remote:crossbeam-utils-0.3.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__deflate__0_7_18",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/deflate/deflate-0.7.18.crate",
        type = "tar.gz",
        strip_prefix = "deflate-0.7.18",
        build_file = "//third_party/cargo/remote:deflate-0.7.18.BUILD"
    )

    native.new_http_archive(
        name = "raze__derivative__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/derivative/derivative-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "derivative-1.0.0",
        build_file = "//third_party/cargo/remote:derivative-1.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__derive_new__0_5_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/derive-new/derive-new-0.5.4.crate",
        type = "tar.gz",
        strip_prefix = "derive-new-0.5.4",
        build_file = "//third_party/cargo/remote:derive-new-0.5.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__derive_builder__0_5_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/derive_builder/derive_builder-0.5.1.crate",
        type = "tar.gz",
        strip_prefix = "derive_builder-0.5.1",
        build_file = "//third_party/cargo/remote:derive_builder-0.5.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__derive_builder_core__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/derive_builder_core/derive_builder_core-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "derive_builder_core-0.2.0",
        build_file = "//third_party/cargo/remote:derive_builder_core-0.2.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__diff__0_1_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/diff/diff-0.1.11.crate",
        type = "tar.gz",
        strip_prefix = "diff-0.1.11",
        build_file = "//third_party/cargo/remote:diff-0.1.11.BUILD"
    )

    native.new_http_archive(
        name = "raze__dtoa__0_4_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/dtoa/dtoa-0.4.2.crate",
        type = "tar.gz",
        strip_prefix = "dtoa-0.4.2",
        build_file = "//third_party/cargo/remote:dtoa-0.4.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__dylib__0_0_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/dylib/dylib-0.0.3.crate",
        type = "tar.gz",
        strip_prefix = "dylib-0.0.3",
        build_file = "//third_party/cargo/remote:dylib-0.0.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__either__1_5_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/either/either-1.5.0.crate",
        type = "tar.gz",
        strip_prefix = "either-1.5.0",
        build_file = "//third_party/cargo/remote:either-1.5.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding__0_2_33",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding/encoding-0.2.33.crate",
        type = "tar.gz",
        strip_prefix = "encoding-0.2.33",
        build_file = "//third_party/cargo/remote:encoding-0.2.33.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding_index_japanese__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-japanese/encoding-index-japanese-1.20141219.5.crate",
        type = "tar.gz",
        strip_prefix = "encoding-index-japanese-1.20141219.5",
        build_file = "//third_party/cargo/remote:encoding-index-japanese-1.20141219.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding_index_korean__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-korean/encoding-index-korean-1.20141219.5.crate",
        type = "tar.gz",
        strip_prefix = "encoding-index-korean-1.20141219.5",
        build_file = "//third_party/cargo/remote:encoding-index-korean-1.20141219.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding_index_simpchinese__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-simpchinese/encoding-index-simpchinese-1.20141219.5.crate",
        type = "tar.gz",
        strip_prefix = "encoding-index-simpchinese-1.20141219.5",
        build_file = "//third_party/cargo/remote:encoding-index-simpchinese-1.20141219.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding_index_singlebyte__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-singlebyte/encoding-index-singlebyte-1.20141219.5.crate",
        type = "tar.gz",
        strip_prefix = "encoding-index-singlebyte-1.20141219.5",
        build_file = "//third_party/cargo/remote:encoding-index-singlebyte-1.20141219.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding_index_tradchinese__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-tradchinese/encoding-index-tradchinese-1.20141219.5.crate",
        type = "tar.gz",
        strip_prefix = "encoding-index-tradchinese-1.20141219.5",
        build_file = "//third_party/cargo/remote:encoding-index-tradchinese-1.20141219.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding_index_tests__0_1_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding_index_tests/encoding_index_tests-0.1.4.crate",
        type = "tar.gz",
        strip_prefix = "encoding_index_tests-0.1.4",
        build_file = "//third_party/cargo/remote:encoding_index_tests-0.1.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__env_logger__0_4_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/env_logger/env_logger-0.4.3.crate",
        type = "tar.gz",
        strip_prefix = "env_logger-0.4.3",
        build_file = "//third_party/cargo/remote:env_logger-0.4.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__env_logger__0_5_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/env_logger/env_logger-0.5.10.crate",
        type = "tar.gz",
        strip_prefix = "env_logger-0.5.10",
        build_file = "//third_party/cargo/remote:env_logger-0.5.10.BUILD"
    )

    native.new_http_archive(
        name = "raze__error_chain__0_11_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/error-chain/error-chain-0.11.0.crate",
        type = "tar.gz",
        strip_prefix = "error-chain-0.11.0",
        build_file = "//third_party/cargo/remote:error-chain-0.11.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__fern__0_4_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fern/fern-0.4.4.crate",
        type = "tar.gz",
        strip_prefix = "fern-0.4.4",
        build_file = "//third_party/cargo/remote:fern-0.4.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__fnv__1_0_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fnv/fnv-1.0.6.crate",
        type = "tar.gz",
        strip_prefix = "fnv-1.0.6",
        build_file = "//third_party/cargo/remote:fnv-1.0.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__fuchsia_zircon__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon/fuchsia-zircon-0.3.3.crate",
        type = "tar.gz",
        strip_prefix = "fuchsia-zircon-0.3.3",
        build_file = "//third_party/cargo/remote:fuchsia-zircon-0.3.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__fuchsia_zircon_sys__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon-sys/fuchsia-zircon-sys-0.3.3.crate",
        type = "tar.gz",
        strip_prefix = "fuchsia-zircon-sys-0.3.3",
        build_file = "//third_party/cargo/remote:fuchsia-zircon-sys-0.3.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__futures__0_1_21",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/futures/futures-0.1.21.crate",
        type = "tar.gz",
        strip_prefix = "futures-0.1.21",
        build_file = "//third_party/cargo/remote:futures-0.1.21.BUILD"
    )

    native.new_http_archive(
        name = "raze__futures_cpupool__0_1_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/futures-cpupool/futures-cpupool-0.1.8.crate",
        type = "tar.gz",
        strip_prefix = "futures-cpupool-0.1.8",
        build_file = "//third_party/cargo/remote:futures-cpupool-0.1.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__getopts__0_2_17",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/getopts/getopts-0.2.17.crate",
        type = "tar.gz",
        strip_prefix = "getopts-0.2.17",
        build_file = "//third_party/cargo/remote:getopts-0.2.17.BUILD"
    )

    native.new_http_archive(
        name = "raze__glob__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/glob/glob-0.2.11.crate",
        type = "tar.gz",
        strip_prefix = "glob-0.2.11",
        build_file = "//third_party/cargo/remote:glob-0.2.11.BUILD"
    )

    native.new_http_archive(
        name = "raze__gnuplot__0_0_24",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/gnuplot/gnuplot-0.0.24.crate",
        type = "tar.gz",
        strip_prefix = "gnuplot-0.0.24",
        build_file = "//third_party/cargo/remote:gnuplot-0.0.24.BUILD"
    )

    native.new_http_archive(
        name = "raze__heatmap__0_6_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/heatmap/heatmap-0.6.6.crate",
        type = "tar.gz",
        strip_prefix = "heatmap-0.6.6",
        build_file = "//third_party/cargo/remote:heatmap-0.6.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__hibitset__0_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hibitset/hibitset-0.3.2.crate",
        type = "tar.gz",
        strip_prefix = "hibitset-0.3.2",
        build_file = "//third_party/cargo/remote:hibitset-0.3.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__histogram__0_6_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/histogram/histogram-0.6.9.crate",
        type = "tar.gz",
        strip_prefix = "histogram-0.6.9",
        build_file = "//third_party/cargo/remote:histogram-0.6.9.BUILD"
    )

    native.new_http_archive(
        name = "raze__hsl__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hsl/hsl-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "hsl-0.1.1",
        build_file = "//third_party/cargo/remote:hsl-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__httparse__1_2_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/httparse/httparse-1.2.4.crate",
        type = "tar.gz",
        strip_prefix = "httparse-1.2.4",
        build_file = "//third_party/cargo/remote:httparse-1.2.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__humantime__1_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/humantime/humantime-1.1.1.crate",
        type = "tar.gz",
        strip_prefix = "humantime-1.1.1",
        build_file = "//third_party/cargo/remote:humantime-1.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__hyper__0_11_27",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hyper/hyper-0.11.27.crate",
        type = "tar.gz",
        strip_prefix = "hyper-0.11.27",
        build_file = "//third_party/cargo/remote:hyper-0.11.27.BUILD"
    )

    native.new_http_archive(
        name = "raze__inflate__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/inflate/inflate-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "inflate-0.2.0",
        build_file = "//third_party/cargo/remote:inflate-0.2.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__inflate__0_3_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/inflate/inflate-0.3.4.crate",
        type = "tar.gz",
        strip_prefix = "inflate-0.3.4",
        build_file = "//third_party/cargo/remote:inflate-0.3.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__iovec__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/iovec/iovec-0.1.2.crate",
        type = "tar.gz",
        strip_prefix = "iovec-0.1.2",
        build_file = "//third_party/cargo/remote:iovec-0.1.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__itertools__0_5_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itertools/itertools-0.5.10.crate",
        type = "tar.gz",
        strip_prefix = "itertools-0.5.10",
        build_file = "//third_party/cargo/remote:itertools-0.5.10.BUILD"
    )

    native.new_http_archive(
        name = "raze__itertools__0_6_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itertools/itertools-0.6.5.crate",
        type = "tar.gz",
        strip_prefix = "itertools-0.6.5",
        build_file = "//third_party/cargo/remote:itertools-0.6.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__itertools__0_7_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itertools/itertools-0.7.8.crate",
        type = "tar.gz",
        strip_prefix = "itertools-0.7.8",
        build_file = "//third_party/cargo/remote:itertools-0.7.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__itoa__0_4_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itoa/itoa-0.4.1.crate",
        type = "tar.gz",
        strip_prefix = "itoa-0.4.1",
        build_file = "//third_party/cargo/remote:itoa-0.4.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__kernel32_sys__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/kernel32-sys/kernel32-sys-0.2.2.crate",
        type = "tar.gz",
        strip_prefix = "kernel32-sys-0.2.2",
        build_file = "//third_party/cargo/remote:kernel32-sys-0.2.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__language_tags__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/language-tags/language-tags-0.2.2.crate",
        type = "tar.gz",
        strip_prefix = "language-tags-0.2.2",
        build_file = "//third_party/cargo/remote:language-tags-0.2.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__lazy_static__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-0.2.11.crate",
        type = "tar.gz",
        strip_prefix = "lazy_static-0.2.11",
        build_file = "//third_party/cargo/remote:lazy_static-0.2.11.BUILD"
    )

    native.new_http_archive(
        name = "raze__lazy_static__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "lazy_static-1.0.0",
        build_file = "//third_party/cargo/remote:lazy_static-1.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__lazycell__0_6_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazycell/lazycell-0.6.0.crate",
        type = "tar.gz",
        strip_prefix = "lazycell-0.6.0",
        build_file = "//third_party/cargo/remote:lazycell-0.6.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__libc__0_2_41",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.41.crate",
        type = "tar.gz",
        strip_prefix = "libc-0.2.41",
        build_file = "//third_party/cargo/remote:libc-0.2.41.BUILD"
    )

    native.new_http_archive(
        name = "raze__libloading__0_5_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libloading/libloading-0.5.0.crate",
        type = "tar.gz",
        strip_prefix = "libloading-0.5.0",
        build_file = "//third_party/cargo/remote:libloading-0.5.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__log__0_3_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.3.9.crate",
        type = "tar.gz",
        strip_prefix = "log-0.3.9",
        build_file = "//third_party/cargo/remote:log-0.3.9.BUILD"
    )

    native.new_http_archive(
        name = "raze__log__0_4_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.4.1.crate",
        type = "tar.gz",
        strip_prefix = "log-0.4.1",
        build_file = "//third_party/cargo/remote:log-0.4.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__matches__0_1_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/matches/matches-0.1.6.crate",
        type = "tar.gz",
        strip_prefix = "matches-0.1.6",
        build_file = "//third_party/cargo/remote:matches-0.1.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__memchr__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-1.0.2.crate",
        type = "tar.gz",
        strip_prefix = "memchr-1.0.2",
        build_file = "//third_party/cargo/remote:memchr-1.0.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__memchr__2_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-2.0.1.crate",
        type = "tar.gz",
        strip_prefix = "memchr-2.0.1",
        build_file = "//third_party/cargo/remote:memchr-2.0.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__memoffset__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memoffset/memoffset-0.2.1.crate",
        type = "tar.gz",
        strip_prefix = "memoffset-0.2.1",
        build_file = "//third_party/cargo/remote:memoffset-0.2.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__mime__0_3_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mime/mime-0.3.7.crate",
        type = "tar.gz",
        strip_prefix = "mime-0.3.7",
        build_file = "//third_party/cargo/remote:mime-0.3.7.BUILD"
    )

    native.new_http_archive(
        name = "raze__mio__0_6_14",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mio/mio-0.6.14.crate",
        type = "tar.gz",
        strip_prefix = "mio-0.6.14",
        build_file = "//third_party/cargo/remote:mio-0.6.14.BUILD"
    )

    native.new_http_archive(
        name = "raze__miow__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/miow/miow-0.2.1.crate",
        type = "tar.gz",
        strip_prefix = "miow-0.2.1",
        build_file = "//third_party/cargo/remote:miow-0.2.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__mopa__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mopa/mopa-0.2.2.crate",
        type = "tar.gz",
        strip_prefix = "mopa-0.2.2",
        build_file = "//third_party/cargo/remote:mopa-0.2.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__mpmc__0_1_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mpmc/mpmc-0.1.5.crate",
        type = "tar.gz",
        strip_prefix = "mpmc-0.1.5",
        build_file = "//third_party/cargo/remote:mpmc-0.1.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__net2__0_2_32",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/net2/net2-0.2.32.crate",
        type = "tar.gz",
        strip_prefix = "net2-0.2.32",
        build_file = "//third_party/cargo/remote:net2-0.2.32.BUILD"
    )

    native.new_http_archive(
        name = "raze__nodrop__0_1_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/nodrop/nodrop-0.1.12.crate",
        type = "tar.gz",
        strip_prefix = "nodrop-0.1.12",
        build_file = "//third_party/cargo/remote:nodrop-0.1.12.BUILD"
    )

    native.new_http_archive(
        name = "raze__nom__3_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/nom/nom-3.2.1.crate",
        type = "tar.gz",
        strip_prefix = "nom-3.2.1",
        build_file = "//third_party/cargo/remote:nom-3.2.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__num__0_1_42",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num/num-0.1.42.crate",
        type = "tar.gz",
        strip_prefix = "num-0.1.42",
        build_file = "//third_party/cargo/remote:num-0.1.42.BUILD"
    )

    native.new_http_archive(
        name = "raze__num_integer__0_1_38",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-integer/num-integer-0.1.38.crate",
        type = "tar.gz",
        strip_prefix = "num-integer-0.1.38",
        build_file = "//third_party/cargo/remote:num-integer-0.1.38.BUILD"
    )

    native.new_http_archive(
        name = "raze__num_iter__0_1_37",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-iter/num-iter-0.1.37.crate",
        type = "tar.gz",
        strip_prefix = "num-iter-0.1.37",
        build_file = "//third_party/cargo/remote:num-iter-0.1.37.BUILD"
    )

    native.new_http_archive(
        name = "raze__num_traits__0_1_43",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-traits/num-traits-0.1.43.crate",
        type = "tar.gz",
        strip_prefix = "num-traits-0.1.43",
        build_file = "//third_party/cargo/remote:num-traits-0.1.43.BUILD"
    )

    native.new_http_archive(
        name = "raze__num_traits__0_2_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-traits/num-traits-0.2.4.crate",
        type = "tar.gz",
        strip_prefix = "num-traits-0.2.4",
        build_file = "//third_party/cargo/remote:num-traits-0.2.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__num_cpus__1_8_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num_cpus/num_cpus-1.8.0.crate",
        type = "tar.gz",
        strip_prefix = "num_cpus-1.8.0",
        build_file = "//third_party/cargo/remote:num_cpus-1.8.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__odds__0_2_26",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/odds/odds-0.2.26.crate",
        type = "tar.gz",
        strip_prefix = "odds-0.2.26",
        build_file = "//third_party/cargo/remote:odds-0.2.26.BUILD"
    )

    native.new_http_archive(
        name = "raze__owning_ref__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/owning_ref/owning_ref-0.3.3.crate",
        type = "tar.gz",
        strip_prefix = "owning_ref-0.3.3",
        build_file = "//third_party/cargo/remote:owning_ref-0.3.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__parking_lot__0_5_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/parking_lot/parking_lot-0.5.5.crate",
        type = "tar.gz",
        strip_prefix = "parking_lot-0.5.5",
        build_file = "//third_party/cargo/remote:parking_lot-0.5.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__parking_lot_core__0_2_14",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/parking_lot_core/parking_lot_core-0.2.14.crate",
        type = "tar.gz",
        strip_prefix = "parking_lot_core-0.2.14",
        build_file = "//third_party/cargo/remote:parking_lot_core-0.2.14.BUILD"
    )

    native.new_http_archive(
        name = "raze__peeking_take_while__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/peeking_take_while/peeking_take_while-0.1.2.crate",
        type = "tar.gz",
        strip_prefix = "peeking_take_while-0.1.2",
        build_file = "//third_party/cargo/remote:peeking_take_while-0.1.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__percent_encoding__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/percent-encoding/percent-encoding-1.0.1.crate",
        type = "tar.gz",
        strip_prefix = "percent-encoding-1.0.1",
        build_file = "//third_party/cargo/remote:percent-encoding-1.0.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__pkg_config__0_3_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/pkg-config/pkg-config-0.3.11.crate",
        type = "tar.gz",
        strip_prefix = "pkg-config-0.3.11",
        build_file = "//third_party/cargo/remote:pkg-config-0.3.11.BUILD"
    )

    native.new_http_archive(
        name = "raze__png__0_11_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/png/png-0.11.0.crate",
        type = "tar.gz",
        strip_prefix = "png-0.11.0",
        build_file = "//third_party/cargo/remote:png-0.11.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__png__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/png/png-0.7.0.crate",
        type = "tar.gz",
        strip_prefix = "png-0.7.0",
        build_file = "//third_party/cargo/remote:png-0.7.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__proc_macro2__0_3_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/proc-macro2/proc-macro2-0.3.8.crate",
        type = "tar.gz",
        strip_prefix = "proc-macro2-0.3.8",
        build_file = "//third_party/cargo/remote:proc-macro2-0.3.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__proc_macro2__0_4_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/proc-macro2/proc-macro2-0.4.3.crate",
        type = "tar.gz",
        strip_prefix = "proc-macro2-0.4.3",
        build_file = "//third_party/cargo/remote:proc-macro2-0.4.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__protobuf__1_7_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/protobuf/protobuf-1.7.1.crate",
        type = "tar.gz",
        strip_prefix = "protobuf-1.7.1",
        build_file = "//third_party/cargo/remote:protobuf-1.7.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__pulse__0_5_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/pulse/pulse-0.5.3.crate",
        type = "tar.gz",
        strip_prefix = "pulse-0.5.3",
        build_file = "//third_party/cargo/remote:pulse-0.5.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__quick_error__1_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quick-error/quick-error-1.2.2.crate",
        type = "tar.gz",
        strip_prefix = "quick-error-1.2.2",
        build_file = "//third_party/cargo/remote:quick-error-1.2.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__quote__0_3_15",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-0.3.15.crate",
        type = "tar.gz",
        strip_prefix = "quote-0.3.15",
        build_file = "//third_party/cargo/remote:quote-0.3.15.BUILD"
    )

    native.new_http_archive(
        name = "raze__quote__0_5_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-0.5.2.crate",
        type = "tar.gz",
        strip_prefix = "quote-0.5.2",
        build_file = "//third_party/cargo/remote:quote-0.5.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__quote__0_6_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-0.6.2.crate",
        type = "tar.gz",
        strip_prefix = "quote-0.6.2",
        build_file = "//third_party/cargo/remote:quote-0.6.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__rand__0_3_22",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand/rand-0.3.22.crate",
        type = "tar.gz",
        strip_prefix = "rand-0.3.22",
        build_file = "//third_party/cargo/remote:rand-0.3.22.BUILD"
    )

    native.new_http_archive(
        name = "raze__rand__0_4_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand/rand-0.4.2.crate",
        type = "tar.gz",
        strip_prefix = "rand-0.4.2",
        build_file = "//third_party/cargo/remote:rand-0.4.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__rayon__0_8_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rayon/rayon-0.8.2.crate",
        type = "tar.gz",
        strip_prefix = "rayon-0.8.2",
        build_file = "//third_party/cargo/remote:rayon-0.8.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__rayon_core__1_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rayon-core/rayon-core-1.4.0.crate",
        type = "tar.gz",
        strip_prefix = "rayon-core-1.4.0",
        build_file = "//third_party/cargo/remote:rayon-core-1.4.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__redox_syscall__0_1_38",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_syscall/redox_syscall-0.1.38.crate",
        type = "tar.gz",
        strip_prefix = "redox_syscall-0.1.38",
        build_file = "//third_party/cargo/remote:redox_syscall-0.1.38.BUILD"
    )

    native.new_http_archive(
        name = "raze__redox_termios__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_termios/redox_termios-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "redox_termios-0.1.1",
        build_file = "//third_party/cargo/remote:redox_termios-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__regex__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-0.2.11.crate",
        type = "tar.gz",
        strip_prefix = "regex-0.2.11",
        build_file = "//third_party/cargo/remote:regex-0.2.11.BUILD"
    )

    native.new_http_archive(
        name = "raze__regex__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "regex-1.0.0",
        build_file = "//third_party/cargo/remote:regex-1.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__regex_syntax__0_5_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.5.6.crate",
        type = "tar.gz",
        strip_prefix = "regex-syntax-0.5.6",
        build_file = "//third_party/cargo/remote:regex-syntax-0.5.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__regex_syntax__0_6_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.6.0.crate",
        type = "tar.gz",
        strip_prefix = "regex-syntax-0.6.0",
        build_file = "//third_party/cargo/remote:regex-syntax-0.6.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__relay__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/relay/relay-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "relay-0.1.1",
        build_file = "//third_party/cargo/remote:relay-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__rustc_ap_rustc_cratesio_shim__29_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-rustc_cratesio_shim/rustc-ap-rustc_cratesio_shim-29.0.0.crate",
        type = "tar.gz",
        strip_prefix = "rustc-ap-rustc_cratesio_shim-29.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-rustc_cratesio_shim-29.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__rustc_ap_rustc_data_structures__29_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-rustc_data_structures/rustc-ap-rustc_data_structures-29.0.0.crate",
        type = "tar.gz",
        strip_prefix = "rustc-ap-rustc_data_structures-29.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-rustc_data_structures-29.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__rustc_ap_rustc_errors__29_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-rustc_errors/rustc-ap-rustc_errors-29.0.0.crate",
        type = "tar.gz",
        strip_prefix = "rustc-ap-rustc_errors-29.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-rustc_errors-29.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__rustc_ap_serialize__29_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-serialize/rustc-ap-serialize-29.0.0.crate",
        type = "tar.gz",
        strip_prefix = "rustc-ap-serialize-29.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-serialize-29.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__rustc_ap_syntax__29_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-syntax/rustc-ap-syntax-29.0.0.crate",
        type = "tar.gz",
        strip_prefix = "rustc-ap-syntax-29.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-syntax-29.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__rustc_ap_syntax_pos__29_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-ap-syntax_pos/rustc-ap-syntax_pos-29.0.0.crate",
        type = "tar.gz",
        strip_prefix = "rustc-ap-syntax_pos-29.0.0",
        build_file = "//third_party/cargo/remote:rustc-ap-syntax_pos-29.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__rustc_demangle__0_1_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-demangle/rustc-demangle-0.1.8.crate",
        type = "tar.gz",
        strip_prefix = "rustc-demangle-0.1.8",
        build_file = "//third_party/cargo/remote:rustc-demangle-0.1.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__rustc_serialize__0_3_24",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-serialize/rustc-serialize-0.3.24.crate",
        type = "tar.gz",
        strip_prefix = "rustc-serialize-0.3.24",
        build_file = "//third_party/cargo/remote:rustc-serialize-0.3.24.BUILD"
    )

    native.new_http_archive(
        name = "raze__rustfmt_nightly__0_3_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustfmt-nightly/rustfmt-nightly-0.3.8.crate",
        type = "tar.gz",
        strip_prefix = "rustfmt-nightly-0.3.8",
        build_file = "//third_party/cargo/remote:rustfmt-nightly-0.3.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__rusttype__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rusttype/rusttype-0.1.2.crate",
        type = "tar.gz",
        strip_prefix = "rusttype-0.1.2",
        build_file = "//third_party/cargo/remote:rusttype-0.1.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__safemem__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/safemem/safemem-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "safemem-0.2.0",
        build_file = "//third_party/cargo/remote:safemem-0.2.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__scoped_tls__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/scoped-tls/scoped-tls-0.1.2.crate",
        type = "tar.gz",
        strip_prefix = "scoped-tls-0.1.2",
        build_file = "//third_party/cargo/remote:scoped-tls-0.1.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__scopeguard__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/scopeguard/scopeguard-0.3.3.crate",
        type = "tar.gz",
        strip_prefix = "scopeguard-0.3.3",
        build_file = "//third_party/cargo/remote:scopeguard-0.3.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__sdl2__0_31_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/sdl2/sdl2-0.31.0.crate",
        type = "tar.gz",
        strip_prefix = "sdl2-0.31.0",
        build_file = "//third_party/cargo/remote:sdl2-0.31.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__sdl2_sys__0_31_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/sdl2-sys/sdl2-sys-0.31.0.crate",
        type = "tar.gz",
        strip_prefix = "sdl2-sys-0.31.0",
        build_file = "//third_party/cargo/remote:sdl2-sys-0.31.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__semver__0_8_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/semver/semver-0.8.0.crate",
        type = "tar.gz",
        strip_prefix = "semver-0.8.0",
        build_file = "//third_party/cargo/remote:semver-0.8.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__semver_parser__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/semver-parser/semver-parser-0.7.0.crate",
        type = "tar.gz",
        strip_prefix = "semver-parser-0.7.0",
        build_file = "//third_party/cargo/remote:semver-parser-0.7.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__serde__1_0_62",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde/serde-1.0.62.crate",
        type = "tar.gz",
        strip_prefix = "serde-1.0.62",
        build_file = "//third_party/cargo/remote:serde-1.0.62.BUILD"
    )

    native.new_http_archive(
        name = "raze__serde_derive__1_0_62",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde_derive/serde_derive-1.0.62.crate",
        type = "tar.gz",
        strip_prefix = "serde_derive-1.0.62",
        build_file = "//third_party/cargo/remote:serde_derive-1.0.62.BUILD"
    )

    native.new_http_archive(
        name = "raze__serde_json__1_0_18",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde_json/serde_json-1.0.18.crate",
        type = "tar.gz",
        strip_prefix = "serde_json-1.0.18",
        build_file = "//third_party/cargo/remote:serde_json-1.0.18.BUILD"
    )

    native.new_http_archive(
        name = "raze__shred__0_5_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/shred/shred-0.5.2.crate",
        type = "tar.gz",
        strip_prefix = "shred-0.5.2",
        build_file = "//third_party/cargo/remote:shred-0.5.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__shred_derive__0_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/shred-derive/shred-derive-0.3.0.crate",
        type = "tar.gz",
        strip_prefix = "shred-derive-0.3.0",
        build_file = "//third_party/cargo/remote:shred-derive-0.3.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__slab__0_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/slab/slab-0.3.0.crate",
        type = "tar.gz",
        strip_prefix = "slab-0.3.0",
        build_file = "//third_party/cargo/remote:slab-0.3.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__slab__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/slab/slab-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "slab-0.4.0",
        build_file = "//third_party/cargo/remote:slab-0.4.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__smallvec__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/smallvec/smallvec-0.2.1.crate",
        type = "tar.gz",
        strip_prefix = "smallvec-0.2.1",
        build_file = "//third_party/cargo/remote:smallvec-0.2.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__smallvec__0_4_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/smallvec/smallvec-0.4.4.crate",
        type = "tar.gz",
        strip_prefix = "smallvec-0.4.4",
        build_file = "//third_party/cargo/remote:smallvec-0.4.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__smallvec__0_6_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/smallvec/smallvec-0.6.1.crate",
        type = "tar.gz",
        strip_prefix = "smallvec-0.6.1",
        build_file = "//third_party/cargo/remote:smallvec-0.6.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__specs__0_10_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/specs/specs-0.10.0.crate",
        type = "tar.gz",
        strip_prefix = "specs-0.10.0",
        build_file = "//third_party/cargo/remote:specs-0.10.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__stable_deref_trait__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/stable_deref_trait/stable_deref_trait-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "stable_deref_trait-1.0.0",
        build_file = "//third_party/cargo/remote:stable_deref_trait-1.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__stb_truetype__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/stb_truetype/stb_truetype-0.1.2.crate",
        type = "tar.gz",
        strip_prefix = "stb_truetype-0.1.2",
        build_file = "//third_party/cargo/remote:stb_truetype-0.1.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__strsim__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/strsim/strsim-0.7.0.crate",
        type = "tar.gz",
        strip_prefix = "strsim-0.7.0",
        build_file = "//third_party/cargo/remote:strsim-0.7.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__syn__0_10_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.10.8.crate",
        type = "tar.gz",
        strip_prefix = "syn-0.10.8",
        build_file = "//third_party/cargo/remote:syn-0.10.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__syn__0_11_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.11.11.crate",
        type = "tar.gz",
        strip_prefix = "syn-0.11.11",
        build_file = "//third_party/cargo/remote:syn-0.11.11.BUILD"
    )

    native.new_http_archive(
        name = "raze__syn__0_13_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.13.11.crate",
        type = "tar.gz",
        strip_prefix = "syn-0.13.11",
        build_file = "//third_party/cargo/remote:syn-0.13.11.BUILD"
    )

    native.new_http_archive(
        name = "raze__syn__0_14_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.14.0.crate",
        type = "tar.gz",
        strip_prefix = "syn-0.14.0",
        build_file = "//third_party/cargo/remote:syn-0.14.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__synom__0_11_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/synom/synom-0.11.3.crate",
        type = "tar.gz",
        strip_prefix = "synom-0.11.3",
        build_file = "//third_party/cargo/remote:synom-0.11.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__take__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/take/take-0.1.0.crate",
        type = "tar.gz",
        strip_prefix = "take-0.1.0",
        build_file = "//third_party/cargo/remote:take-0.1.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__term__0_4_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/term/term-0.4.6.crate",
        type = "tar.gz",
        strip_prefix = "term-0.4.6",
        build_file = "//third_party/cargo/remote:term-0.4.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__termcolor__0_3_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termcolor/termcolor-0.3.6.crate",
        type = "tar.gz",
        strip_prefix = "termcolor-0.3.6",
        build_file = "//third_party/cargo/remote:termcolor-0.3.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__termion__1_5_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termion/termion-1.5.1.crate",
        type = "tar.gz",
        strip_prefix = "termion-1.5.1",
        build_file = "//third_party/cargo/remote:termion-1.5.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__textwrap__0_9_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/textwrap/textwrap-0.9.0.crate",
        type = "tar.gz",
        strip_prefix = "textwrap-0.9.0",
        build_file = "//third_party/cargo/remote:textwrap-0.9.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__thread_local__0_3_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-0.3.5.crate",
        type = "tar.gz",
        strip_prefix = "thread_local-0.3.5",
        build_file = "//third_party/cargo/remote:thread_local-0.3.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__tic__0_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tic/tic-0.3.0.crate",
        type = "tar.gz",
        strip_prefix = "tic-0.3.0",
        build_file = "//third_party/cargo/remote:tic-0.3.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__time__0_1_40",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/time/time-0.1.40.crate",
        type = "tar.gz",
        strip_prefix = "time-0.1.40",
        build_file = "//third_party/cargo/remote:time-0.1.40.BUILD"
    )

    native.new_http_archive(
        name = "raze__tiny_http__0_5_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tiny_http/tiny_http-0.5.9.crate",
        type = "tar.gz",
        strip_prefix = "tiny_http-0.5.9",
        build_file = "//third_party/cargo/remote:tiny_http-0.5.9.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio__0_1_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio/tokio-0.1.6.crate",
        type = "tar.gz",
        strip_prefix = "tokio-0.1.6",
        build_file = "//third_party/cargo/remote:tokio-0.1.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_core__0_1_17",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-core/tokio-core-0.1.17.crate",
        type = "tar.gz",
        strip_prefix = "tokio-core-0.1.17",
        build_file = "//third_party/cargo/remote:tokio-core-0.1.17.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_executor__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-executor/tokio-executor-0.1.2.crate",
        type = "tar.gz",
        strip_prefix = "tokio-executor-0.1.2",
        build_file = "//third_party/cargo/remote:tokio-executor-0.1.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_fs__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-fs/tokio-fs-0.1.0.crate",
        type = "tar.gz",
        strip_prefix = "tokio-fs-0.1.0",
        build_file = "//third_party/cargo/remote:tokio-fs-0.1.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_io__0_1_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-io/tokio-io-0.1.6.crate",
        type = "tar.gz",
        strip_prefix = "tokio-io-0.1.6",
        build_file = "//third_party/cargo/remote:tokio-io-0.1.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_proto__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-proto/tokio-proto-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "tokio-proto-0.1.1",
        build_file = "//third_party/cargo/remote:tokio-proto-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_reactor__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-reactor/tokio-reactor-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "tokio-reactor-0.1.1",
        build_file = "//third_party/cargo/remote:tokio-reactor-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_service__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-service/tokio-service-0.1.0.crate",
        type = "tar.gz",
        strip_prefix = "tokio-service-0.1.0",
        build_file = "//third_party/cargo/remote:tokio-service-0.1.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_tcp__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-tcp/tokio-tcp-0.1.0.crate",
        type = "tar.gz",
        strip_prefix = "tokio-tcp-0.1.0",
        build_file = "//third_party/cargo/remote:tokio-tcp-0.1.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_threadpool__0_1_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-threadpool/tokio-threadpool-0.1.3.crate",
        type = "tar.gz",
        strip_prefix = "tokio-threadpool-0.1.3",
        build_file = "//third_party/cargo/remote:tokio-threadpool-0.1.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_timer__0_2_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-timer/tokio-timer-0.2.3.crate",
        type = "tar.gz",
        strip_prefix = "tokio-timer-0.2.3",
        build_file = "//third_party/cargo/remote:tokio-timer-0.2.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_udp__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-udp/tokio-udp-0.1.0.crate",
        type = "tar.gz",
        strip_prefix = "tokio-udp-0.1.0",
        build_file = "//third_party/cargo/remote:tokio-udp-0.1.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__toml__0_4_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/toml/toml-0.4.6.crate",
        type = "tar.gz",
        strip_prefix = "toml-0.4.6",
        build_file = "//third_party/cargo/remote:toml-0.4.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__try_lock__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/try-lock/try-lock-0.1.0.crate",
        type = "tar.gz",
        strip_prefix = "try-lock-0.1.0",
        build_file = "//third_party/cargo/remote:try-lock-0.1.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__tuple_utils__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tuple_utils/tuple_utils-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "tuple_utils-0.2.0",
        build_file = "//third_party/cargo/remote:tuple_utils-0.2.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__ucd_util__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ucd-util/ucd-util-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "ucd-util-0.1.1",
        build_file = "//third_party/cargo/remote:ucd-util-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__unicase__2_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicase/unicase-2.1.0.crate",
        type = "tar.gz",
        strip_prefix = "unicase-2.1.0",
        build_file = "//third_party/cargo/remote:unicase-2.1.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__unicode_segmentation__1_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-segmentation/unicode-segmentation-1.2.1.crate",
        type = "tar.gz",
        strip_prefix = "unicode-segmentation-1.2.1",
        build_file = "//third_party/cargo/remote:unicode-segmentation-1.2.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__unicode_width__0_1_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-width/unicode-width-0.1.5.crate",
        type = "tar.gz",
        strip_prefix = "unicode-width-0.1.5",
        build_file = "//third_party/cargo/remote:unicode-width-0.1.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__unicode_xid__0_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-xid/unicode-xid-0.0.4.crate",
        type = "tar.gz",
        strip_prefix = "unicode-xid-0.0.4",
        build_file = "//third_party/cargo/remote:unicode-xid-0.0.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__unicode_xid__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-xid/unicode-xid-0.1.0.crate",
        type = "tar.gz",
        strip_prefix = "unicode-xid-0.1.0",
        build_file = "//third_party/cargo/remote:unicode-xid-0.1.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__unreachable__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unreachable/unreachable-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "unreachable-1.0.0",
        build_file = "//third_party/cargo/remote:unreachable-1.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__url__0_2_38",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/url/url-0.2.38.crate",
        type = "tar.gz",
        strip_prefix = "url-0.2.38",
        build_file = "//third_party/cargo/remote:url-0.2.38.BUILD"
    )

    native.new_http_archive(
        name = "raze__utf8_ranges__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/utf8-ranges/utf8-ranges-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "utf8-ranges-1.0.0",
        build_file = "//third_party/cargo/remote:utf8-ranges-1.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__uuid__0_1_18",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/uuid/uuid-0.1.18.crate",
        type = "tar.gz",
        strip_prefix = "uuid-0.1.18",
        build_file = "//third_party/cargo/remote:uuid-0.1.18.BUILD"
    )

    native.new_http_archive(
        name = "raze__vec_map__0_8_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/vec_map/vec_map-0.8.1.crate",
        type = "tar.gz",
        strip_prefix = "vec_map-0.8.1",
        build_file = "//third_party/cargo/remote:vec_map-0.8.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__version_check__0_1_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/version_check/version_check-0.1.3.crate",
        type = "tar.gz",
        strip_prefix = "version_check-0.1.3",
        build_file = "//third_party/cargo/remote:version_check-0.1.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__vk_sys__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/vk-sys/vk-sys-0.3.3.crate",
        type = "tar.gz",
        strip_prefix = "vk-sys-0.3.3",
        build_file = "//third_party/cargo/remote:vk-sys-0.3.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__void__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/void/void-1.0.2.crate",
        type = "tar.gz",
        strip_prefix = "void-1.0.2",
        build_file = "//third_party/cargo/remote:void-1.0.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__want__0_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/want/want-0.0.4.crate",
        type = "tar.gz",
        strip_prefix = "want-0.0.4",
        build_file = "//third_party/cargo/remote:want-0.0.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__waterfall__0_7_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/waterfall/waterfall-0.7.1.crate",
        type = "tar.gz",
        strip_prefix = "waterfall-0.7.1",
        build_file = "//third_party/cargo/remote:waterfall-0.7.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__which__1_0_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/which/which-1.0.5.crate",
        type = "tar.gz",
        strip_prefix = "which-1.0.5",
        build_file = "//third_party/cargo/remote:which-1.0.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__winapi__0_2_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.2.8.crate",
        type = "tar.gz",
        strip_prefix = "winapi-0.2.8",
        build_file = "//third_party/cargo/remote:winapi-0.2.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__winapi__0_3_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.4.crate",
        type = "tar.gz",
        strip_prefix = "winapi-0.3.4",
        build_file = "//third_party/cargo/remote:winapi-0.3.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__winapi_build__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-build/winapi-build-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "winapi-build-0.1.1",
        build_file = "//third_party/cargo/remote:winapi-build-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = "//third_party/cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = "//third_party/cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__wincolor__0_1_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/wincolor/wincolor-0.1.6.crate",
        type = "tar.gz",
        strip_prefix = "wincolor-0.1.6",
        build_file = "//third_party/cargo/remote:wincolor-0.1.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__ws2_32_sys__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ws2_32-sys/ws2_32-sys-0.2.1.crate",
        type = "tar.gz",
        strip_prefix = "ws2_32-sys-0.2.1",
        build_file = "//third_party/cargo/remote:ws2_32-sys-0.2.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__x11_dl__2_17_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/x11-dl/x11-dl-2.17.5.crate",
        type = "tar.gz",
        strip_prefix = "x11-dl-2.17.5",
        build_file = "//third_party/cargo/remote:x11-dl-2.17.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__zcfg__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/zcfg/zcfg-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "zcfg-0.2.0",
        build_file = "//third_party/cargo/remote:zcfg-0.2.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__zcfg_flag_parser__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/zcfg_flag_parser/zcfg_flag_parser-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "zcfg_flag_parser-0.2.0",
        build_file = "//third_party/cargo/remote:zcfg_flag_parser-0.2.0.BUILD"
    )

