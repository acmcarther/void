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
        build_file = "//cargo:adler32-1.0.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__aho_corasick__0_6_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.6.4.crate",
        type = "tar.gz",
        strip_prefix = "aho-corasick-0.6.4",
        build_file = "//cargo:aho-corasick-0.6.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__allan__0_2_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/allan/allan-0.2.4.crate",
        type = "tar.gz",
        strip_prefix = "allan-0.2.4",
        build_file = "//cargo:allan-0.2.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__ansi_term__0_10_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ansi_term/ansi_term-0.10.2.crate",
        type = "tar.gz",
        strip_prefix = "ansi_term-0.10.2",
        build_file = "//cargo:ansi_term-0.10.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__arrayvec__0_3_24",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/arrayvec/arrayvec-0.3.24.crate",
        type = "tar.gz",
        strip_prefix = "arrayvec-0.3.24",
        build_file = "//cargo:arrayvec-0.3.24.BUILD"
    )

    native.new_http_archive(
        name = "raze__ascii__0_7_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ascii/ascii-0.7.1.crate",
        type = "tar.gz",
        strip_prefix = "ascii-0.7.1",
        build_file = "//cargo:ascii-0.7.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__atom__0_3_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atom/atom-0.3.4.crate",
        type = "tar.gz",
        strip_prefix = "atom-0.3.4",
        build_file = "//cargo:atom-0.3.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__atty__0_2_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atty/atty-0.2.6.crate",
        type = "tar.gz",
        strip_prefix = "atty-0.2.6",
        build_file = "//cargo:atty-0.2.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__base64__0_6_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/base64/base64-0.6.0.crate",
        type = "tar.gz",
        strip_prefix = "base64-0.6.0",
        build_file = "//cargo:base64-0.6.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__bindgen__0_32_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bindgen/bindgen-0.32.1.crate",
        type = "tar.gz",
        strip_prefix = "bindgen-0.32.1",
        build_file = "//cargo:bindgen-0.32.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__bitflags__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-0.7.0.crate",
        type = "tar.gz",
        strip_prefix = "bitflags-0.7.0",
        build_file = "//cargo:bitflags-0.7.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__bitflags__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-1.0.1.crate",
        type = "tar.gz",
        strip_prefix = "bitflags-1.0.1",
        build_file = "//cargo:bitflags-1.0.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__byteorder__0_4_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/byteorder/byteorder-0.4.2.crate",
        type = "tar.gz",
        strip_prefix = "byteorder-0.4.2",
        build_file = "//cargo:byteorder-0.4.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__byteorder__1_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/byteorder/byteorder-1.2.1.crate",
        type = "tar.gz",
        strip_prefix = "byteorder-1.2.1",
        build_file = "//cargo:byteorder-1.2.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__bytes__0_4_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bytes/bytes-0.4.5.crate",
        type = "tar.gz",
        strip_prefix = "bytes-0.4.5",
        build_file = "//cargo:bytes-0.4.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__cc__1_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cc/cc-1.0.4.crate",
        type = "tar.gz",
        strip_prefix = "cc-1.0.4",
        build_file = "//cargo:cc-1.0.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__cexpr__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cexpr/cexpr-0.2.2.crate",
        type = "tar.gz",
        strip_prefix = "cexpr-0.2.2",
        build_file = "//cargo:cexpr-0.2.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__cfg_if__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.2.crate",
        type = "tar.gz",
        strip_prefix = "cfg-if-0.1.2",
        build_file = "//cargo:cfg-if-0.1.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__chrono__0_2_25",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/chrono/chrono-0.2.25.crate",
        type = "tar.gz",
        strip_prefix = "chrono-0.2.25",
        build_file = "//cargo:chrono-0.2.25.BUILD"
    )

    native.new_http_archive(
        name = "raze__chrono__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/chrono/chrono-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "chrono-0.4.0",
        build_file = "//cargo:chrono-0.4.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__chunked_transfer__0_3_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/chunked_transfer/chunked_transfer-0.3.1.crate",
        type = "tar.gz",
        strip_prefix = "chunked_transfer-0.3.1",
        build_file = "//cargo:chunked_transfer-0.3.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__clang_sys__0_21_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clang-sys/clang-sys-0.21.1.crate",
        type = "tar.gz",
        strip_prefix = "clang-sys-0.21.1",
        build_file = "//cargo:clang-sys-0.21.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__clap__2_29_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clap/clap-2.29.1.crate",
        type = "tar.gz",
        strip_prefix = "clap-2.29.1",
        build_file = "//cargo:clap-2.29.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__clocksource__0_2_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clocksource/clocksource-0.2.3.crate",
        type = "tar.gz",
        strip_prefix = "clocksource-0.2.3",
        build_file = "//cargo:clocksource-0.2.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__cmake__0_1_29",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cmake/cmake-0.1.29.crate",
        type = "tar.gz",
        strip_prefix = "cmake-0.1.29",
        build_file = "//cargo:cmake-0.1.29.BUILD"
    )

    native.new_http_archive(
        name = "raze__coco__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/coco/coco-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "coco-0.1.1",
        build_file = "//cargo:coco-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__crossbeam__0_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam/crossbeam-0.3.0.crate",
        type = "tar.gz",
        strip_prefix = "crossbeam-0.3.0",
        build_file = "//cargo:crossbeam-0.3.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__deflate__0_7_17",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/deflate/deflate-0.7.17.crate",
        type = "tar.gz",
        strip_prefix = "deflate-0.7.17",
        build_file = "//cargo:deflate-0.7.17.BUILD"
    )

    native.new_http_archive(
        name = "raze__derivative__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/derivative/derivative-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "derivative-1.0.0",
        build_file = "//cargo:derivative-1.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__either__1_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/either/either-1.4.0.crate",
        type = "tar.gz",
        strip_prefix = "either-1.4.0",
        build_file = "//cargo:either-1.4.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding__0_2_33",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding/encoding-0.2.33.crate",
        type = "tar.gz",
        strip_prefix = "encoding-0.2.33",
        build_file = "//cargo:encoding-0.2.33.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding_index_japanese__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-japanese/encoding-index-japanese-1.20141219.5.crate",
        type = "tar.gz",
        strip_prefix = "encoding-index-japanese-1.20141219.5",
        build_file = "//cargo:encoding-index-japanese-1.20141219.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding_index_korean__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-korean/encoding-index-korean-1.20141219.5.crate",
        type = "tar.gz",
        strip_prefix = "encoding-index-korean-1.20141219.5",
        build_file = "//cargo:encoding-index-korean-1.20141219.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding_index_simpchinese__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-simpchinese/encoding-index-simpchinese-1.20141219.5.crate",
        type = "tar.gz",
        strip_prefix = "encoding-index-simpchinese-1.20141219.5",
        build_file = "//cargo:encoding-index-simpchinese-1.20141219.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding_index_singlebyte__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-singlebyte/encoding-index-singlebyte-1.20141219.5.crate",
        type = "tar.gz",
        strip_prefix = "encoding-index-singlebyte-1.20141219.5",
        build_file = "//cargo:encoding-index-singlebyte-1.20141219.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding_index_tradchinese__1_20141219_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding-index-tradchinese/encoding-index-tradchinese-1.20141219.5.crate",
        type = "tar.gz",
        strip_prefix = "encoding-index-tradchinese-1.20141219.5",
        build_file = "//cargo:encoding-index-tradchinese-1.20141219.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__encoding_index_tests__0_1_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/encoding_index_tests/encoding_index_tests-0.1.4.crate",
        type = "tar.gz",
        strip_prefix = "encoding_index_tests-0.1.4",
        build_file = "//cargo:encoding_index_tests-0.1.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__enum_primitive__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/enum_primitive/enum_primitive-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "enum_primitive-0.1.1",
        build_file = "//cargo:enum_primitive-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__env_logger__0_4_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/env_logger/env_logger-0.4.3.crate",
        type = "tar.gz",
        strip_prefix = "env_logger-0.4.3",
        build_file = "//cargo:env_logger-0.4.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__fern__0_4_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fern/fern-0.4.3.crate",
        type = "tar.gz",
        strip_prefix = "fern-0.4.3",
        build_file = "//cargo:fern-0.4.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__fnv__1_0_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fnv/fnv-1.0.6.crate",
        type = "tar.gz",
        strip_prefix = "fnv-1.0.6",
        build_file = "//cargo:fnv-1.0.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__fuchsia_zircon__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon/fuchsia-zircon-0.2.1.crate",
        type = "tar.gz",
        strip_prefix = "fuchsia-zircon-0.2.1",
        build_file = "//cargo:fuchsia-zircon-0.2.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__fuchsia_zircon_sys__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon-sys/fuchsia-zircon-sys-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "fuchsia-zircon-sys-0.2.0",
        build_file = "//cargo:fuchsia-zircon-sys-0.2.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__futures__0_1_17",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/futures/futures-0.1.17.crate",
        type = "tar.gz",
        strip_prefix = "futures-0.1.17",
        build_file = "//cargo:futures-0.1.17.BUILD"
    )

    native.new_http_archive(
        name = "raze__futures_cpupool__0_1_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/futures-cpupool/futures-cpupool-0.1.7.crate",
        type = "tar.gz",
        strip_prefix = "futures-cpupool-0.1.7",
        build_file = "//cargo:futures-cpupool-0.1.7.BUILD"
    )

    native.new_http_archive(
        name = "raze__getopts__0_2_15",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/getopts/getopts-0.2.15.crate",
        type = "tar.gz",
        strip_prefix = "getopts-0.2.15",
        build_file = "//cargo:getopts-0.2.15.BUILD"
    )

    native.new_http_archive(
        name = "raze__glfw__0_20_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/glfw/glfw-0.20.0.crate",
        type = "tar.gz",
        strip_prefix = "glfw-0.20.0",
        build_file = "//cargo:glfw-0.20.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__glfw_sys__3_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/glfw-sys/glfw-sys-3.2.2.crate",
        type = "tar.gz",
        strip_prefix = "glfw-sys-3.2.2",
        build_file = "//cargo:glfw-sys-3.2.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__glob__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/glob/glob-0.2.11.crate",
        type = "tar.gz",
        strip_prefix = "glob-0.2.11",
        build_file = "//cargo:glob-0.2.11.BUILD"
    )

    native.new_http_archive(
        name = "raze__gnuplot__0_0_24",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/gnuplot/gnuplot-0.0.24.crate",
        type = "tar.gz",
        strip_prefix = "gnuplot-0.0.24",
        build_file = "//cargo:gnuplot-0.0.24.BUILD"
    )

    native.new_http_archive(
        name = "raze__heatmap__0_6_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/heatmap/heatmap-0.6.6.crate",
        type = "tar.gz",
        strip_prefix = "heatmap-0.6.6",
        build_file = "//cargo:heatmap-0.6.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__hibitset__0_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hibitset/hibitset-0.3.2.crate",
        type = "tar.gz",
        strip_prefix = "hibitset-0.3.2",
        build_file = "//cargo:hibitset-0.3.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__histogram__0_6_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/histogram/histogram-0.6.8.crate",
        type = "tar.gz",
        strip_prefix = "histogram-0.6.8",
        build_file = "//cargo:histogram-0.6.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__hsl__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hsl/hsl-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "hsl-0.1.1",
        build_file = "//cargo:hsl-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__httparse__1_2_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/httparse/httparse-1.2.3.crate",
        type = "tar.gz",
        strip_prefix = "httparse-1.2.3",
        build_file = "//cargo:httparse-1.2.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__hyper__0_11_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hyper/hyper-0.11.7.crate",
        type = "tar.gz",
        strip_prefix = "hyper-0.11.7",
        build_file = "//cargo:hyper-0.11.7.BUILD"
    )

    native.new_http_archive(
        name = "raze__inflate__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/inflate/inflate-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "inflate-0.2.0",
        build_file = "//cargo:inflate-0.2.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__iovec__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/iovec/iovec-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "iovec-0.1.1",
        build_file = "//cargo:iovec-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__itertools__0_5_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itertools/itertools-0.5.10.crate",
        type = "tar.gz",
        strip_prefix = "itertools-0.5.10",
        build_file = "//cargo:itertools-0.5.10.BUILD"
    )

    native.new_http_archive(
        name = "raze__itertools__0_6_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itertools/itertools-0.6.5.crate",
        type = "tar.gz",
        strip_prefix = "itertools-0.6.5",
        build_file = "//cargo:itertools-0.6.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__itertools__0_7_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itertools/itertools-0.7.4.crate",
        type = "tar.gz",
        strip_prefix = "itertools-0.7.4",
        build_file = "//cargo:itertools-0.7.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__kernel32_sys__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/kernel32-sys/kernel32-sys-0.2.2.crate",
        type = "tar.gz",
        strip_prefix = "kernel32-sys-0.2.2",
        build_file = "//cargo:kernel32-sys-0.2.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__language_tags__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/language-tags/language-tags-0.2.2.crate",
        type = "tar.gz",
        strip_prefix = "language-tags-0.2.2",
        build_file = "//cargo:language-tags-0.2.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__lazy_static__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-0.2.11.crate",
        type = "tar.gz",
        strip_prefix = "lazy_static-0.2.11",
        build_file = "//cargo:lazy_static-0.2.11.BUILD"
    )

    native.new_http_archive(
        name = "raze__lazy_static__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "lazy_static-1.0.0",
        build_file = "//cargo:lazy_static-1.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__lazycell__0_5_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazycell/lazycell-0.5.1.crate",
        type = "tar.gz",
        strip_prefix = "lazycell-0.5.1",
        build_file = "//cargo:lazycell-0.5.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__libc__0_2_34",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.34.crate",
        type = "tar.gz",
        strip_prefix = "libc-0.2.34",
        build_file = "//cargo:libc-0.2.34.BUILD"
    )

    native.new_http_archive(
        name = "raze__libloading__0_4_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libloading/libloading-0.4.3.crate",
        type = "tar.gz",
        strip_prefix = "libloading-0.4.3",
        build_file = "//cargo:libloading-0.4.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__log__0_3_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.3.8.crate",
        type = "tar.gz",
        strip_prefix = "log-0.3.8",
        build_file = "//cargo:log-0.3.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__matches__0_1_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/matches/matches-0.1.6.crate",
        type = "tar.gz",
        strip_prefix = "matches-0.1.6",
        build_file = "//cargo:matches-0.1.6.BUILD"
    )

    native.new_http_archive(
        name = "raze__memchr__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-1.0.2.crate",
        type = "tar.gz",
        strip_prefix = "memchr-1.0.2",
        build_file = "//cargo:memchr-1.0.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__memchr__2_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-2.0.1.crate",
        type = "tar.gz",
        strip_prefix = "memchr-2.0.1",
        build_file = "//cargo:memchr-2.0.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__mime__0_3_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mime/mime-0.3.5.crate",
        type = "tar.gz",
        strip_prefix = "mime-0.3.5",
        build_file = "//cargo:mime-0.3.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__mio__0_6_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mio/mio-0.6.11.crate",
        type = "tar.gz",
        strip_prefix = "mio-0.6.11",
        build_file = "//cargo:mio-0.6.11.BUILD"
    )

    native.new_http_archive(
        name = "raze__miow__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/miow/miow-0.2.1.crate",
        type = "tar.gz",
        strip_prefix = "miow-0.2.1",
        build_file = "//cargo:miow-0.2.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__mopa__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mopa/mopa-0.2.2.crate",
        type = "tar.gz",
        strip_prefix = "mopa-0.2.2",
        build_file = "//cargo:mopa-0.2.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__mpmc__0_1_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mpmc/mpmc-0.1.5.crate",
        type = "tar.gz",
        strip_prefix = "mpmc-0.1.5",
        build_file = "//cargo:mpmc-0.1.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__net2__0_2_31",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/net2/net2-0.2.31.crate",
        type = "tar.gz",
        strip_prefix = "net2-0.2.31",
        build_file = "//cargo:net2-0.2.31.BUILD"
    )

    native.new_http_archive(
        name = "raze__nodrop__0_1_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/nodrop/nodrop-0.1.12.crate",
        type = "tar.gz",
        strip_prefix = "nodrop-0.1.12",
        build_file = "//cargo:nodrop-0.1.12.BUILD"
    )

    native.new_http_archive(
        name = "raze__nom__1_2_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/nom/nom-1.2.4.crate",
        type = "tar.gz",
        strip_prefix = "nom-1.2.4",
        build_file = "//cargo:nom-1.2.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__nom__3_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/nom/nom-3.2.1.crate",
        type = "tar.gz",
        strip_prefix = "nom-3.2.1",
        build_file = "//cargo:nom-3.2.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__num__0_1_41",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num/num-0.1.41.crate",
        type = "tar.gz",
        strip_prefix = "num-0.1.41",
        build_file = "//cargo:num-0.1.41.BUILD"
    )

    native.new_http_archive(
        name = "raze__num_bigint__0_1_41",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-bigint/num-bigint-0.1.41.crate",
        type = "tar.gz",
        strip_prefix = "num-bigint-0.1.41",
        build_file = "//cargo:num-bigint-0.1.41.BUILD"
    )

    native.new_http_archive(
        name = "raze__num_complex__0_1_41",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-complex/num-complex-0.1.41.crate",
        type = "tar.gz",
        strip_prefix = "num-complex-0.1.41",
        build_file = "//cargo:num-complex-0.1.41.BUILD"
    )

    native.new_http_archive(
        name = "raze__num_integer__0_1_35",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-integer/num-integer-0.1.35.crate",
        type = "tar.gz",
        strip_prefix = "num-integer-0.1.35",
        build_file = "//cargo:num-integer-0.1.35.BUILD"
    )

    native.new_http_archive(
        name = "raze__num_iter__0_1_34",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-iter/num-iter-0.1.34.crate",
        type = "tar.gz",
        strip_prefix = "num-iter-0.1.34",
        build_file = "//cargo:num-iter-0.1.34.BUILD"
    )

    native.new_http_archive(
        name = "raze__num_rational__0_1_40",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-rational/num-rational-0.1.40.crate",
        type = "tar.gz",
        strip_prefix = "num-rational-0.1.40",
        build_file = "//cargo:num-rational-0.1.40.BUILD"
    )

    native.new_http_archive(
        name = "raze__num_traits__0_1_41",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-traits/num-traits-0.1.41.crate",
        type = "tar.gz",
        strip_prefix = "num-traits-0.1.41",
        build_file = "//cargo:num-traits-0.1.41.BUILD"
    )

    native.new_http_archive(
        name = "raze__num_cpus__1_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num_cpus/num_cpus-1.7.0.crate",
        type = "tar.gz",
        strip_prefix = "num_cpus-1.7.0",
        build_file = "//cargo:num_cpus-1.7.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__odds__0_2_26",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/odds/odds-0.2.26.crate",
        type = "tar.gz",
        strip_prefix = "odds-0.2.26",
        build_file = "//cargo:odds-0.2.26.BUILD"
    )

    native.new_http_archive(
        name = "raze__peeking_take_while__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/peeking_take_while/peeking_take_while-0.1.2.crate",
        type = "tar.gz",
        strip_prefix = "peeking_take_while-0.1.2",
        build_file = "//cargo:peeking_take_while-0.1.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__percent_encoding__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/percent-encoding/percent-encoding-1.0.1.crate",
        type = "tar.gz",
        strip_prefix = "percent-encoding-1.0.1",
        build_file = "//cargo:percent-encoding-1.0.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__png__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/png/png-0.7.0.crate",
        type = "tar.gz",
        strip_prefix = "png-0.7.0",
        build_file = "//cargo:png-0.7.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__pulse__0_5_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/pulse/pulse-0.5.3.crate",
        type = "tar.gz",
        strip_prefix = "pulse-0.5.3",
        build_file = "//cargo:pulse-0.5.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__quote__0_3_15",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-0.3.15.crate",
        type = "tar.gz",
        strip_prefix = "quote-0.3.15",
        build_file = "//cargo:quote-0.3.15.BUILD"
    )

    native.new_http_archive(
        name = "raze__rand__0_3_18",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand/rand-0.3.18.crate",
        type = "tar.gz",
        strip_prefix = "rand-0.3.18",
        build_file = "//cargo:rand-0.3.18.BUILD"
    )

    native.new_http_archive(
        name = "raze__rayon__0_8_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rayon/rayon-0.8.2.crate",
        type = "tar.gz",
        strip_prefix = "rayon-0.8.2",
        build_file = "//cargo:rayon-0.8.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__rayon_core__1_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rayon-core/rayon-core-1.3.0.crate",
        type = "tar.gz",
        strip_prefix = "rayon-core-1.3.0",
        build_file = "//cargo:rayon-core-1.3.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__redox_syscall__0_1_32",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_syscall/redox_syscall-0.1.32.crate",
        type = "tar.gz",
        strip_prefix = "redox_syscall-0.1.32",
        build_file = "//cargo:redox_syscall-0.1.32.BUILD"
    )

    native.new_http_archive(
        name = "raze__redox_termios__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_termios/redox_termios-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "redox_termios-0.1.1",
        build_file = "//cargo:redox_termios-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__regex__0_2_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-0.2.5.crate",
        type = "tar.gz",
        strip_prefix = "regex-0.2.5",
        build_file = "//cargo:regex-0.2.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__regex_syntax__0_4_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.4.2.crate",
        type = "tar.gz",
        strip_prefix = "regex-syntax-0.4.2",
        build_file = "//cargo:regex-syntax-0.4.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__relay__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/relay/relay-0.1.0.crate",
        type = "tar.gz",
        strip_prefix = "relay-0.1.0",
        build_file = "//cargo:relay-0.1.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__rustc_serialize__0_3_24",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc-serialize/rustc-serialize-0.3.24.crate",
        type = "tar.gz",
        strip_prefix = "rustc-serialize-0.3.24",
        build_file = "//cargo:rustc-serialize-0.3.24.BUILD"
    )

    native.new_http_archive(
        name = "raze__rusttype__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rusttype/rusttype-0.1.2.crate",
        type = "tar.gz",
        strip_prefix = "rusttype-0.1.2",
        build_file = "//cargo:rusttype-0.1.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__safemem__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/safemem/safemem-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "safemem-0.2.0",
        build_file = "//cargo:safemem-0.2.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__scoped_tls__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/scoped-tls/scoped-tls-0.1.0.crate",
        type = "tar.gz",
        strip_prefix = "scoped-tls-0.1.0",
        build_file = "//cargo:scoped-tls-0.1.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__scopeguard__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/scopeguard/scopeguard-0.3.3.crate",
        type = "tar.gz",
        strip_prefix = "scopeguard-0.3.3",
        build_file = "//cargo:scopeguard-0.3.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__sdl2__0_31_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/sdl2/sdl2-0.31.0.crate",
        type = "tar.gz",
        strip_prefix = "sdl2-0.31.0",
        build_file = "//cargo:sdl2-0.31.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__sdl2_sys__0_31_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/sdl2-sys/sdl2-sys-0.31.0.crate",
        type = "tar.gz",
        strip_prefix = "sdl2-sys-0.31.0",
        build_file = "//cargo:sdl2-sys-0.31.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__semver__0_2_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/semver/semver-0.2.3.crate",
        type = "tar.gz",
        strip_prefix = "semver-0.2.3",
        build_file = "//cargo:semver-0.2.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__shred__0_5_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/shred/shred-0.5.0.crate",
        type = "tar.gz",
        strip_prefix = "shred-0.5.0",
        build_file = "//cargo:shred-0.5.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__shred_derive__0_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/shred-derive/shred-derive-0.3.0.crate",
        type = "tar.gz",
        strip_prefix = "shred-derive-0.3.0",
        build_file = "//cargo:shred-derive-0.3.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__slab__0_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/slab/slab-0.3.0.crate",
        type = "tar.gz",
        strip_prefix = "slab-0.3.0",
        build_file = "//cargo:slab-0.3.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__slab__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/slab/slab-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "slab-0.4.0",
        build_file = "//cargo:slab-0.4.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__smallvec__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/smallvec/smallvec-0.2.1.crate",
        type = "tar.gz",
        strip_prefix = "smallvec-0.2.1",
        build_file = "//cargo:smallvec-0.2.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__smallvec__0_4_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/smallvec/smallvec-0.4.4.crate",
        type = "tar.gz",
        strip_prefix = "smallvec-0.4.4",
        build_file = "//cargo:smallvec-0.4.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__specs__0_10_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/specs/specs-0.10.0.crate",
        type = "tar.gz",
        strip_prefix = "specs-0.10.0",
        build_file = "//cargo:specs-0.10.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__stb_truetype__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/stb_truetype/stb_truetype-0.1.2.crate",
        type = "tar.gz",
        strip_prefix = "stb_truetype-0.1.2",
        build_file = "//cargo:stb_truetype-0.1.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__strsim__0_6_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/strsim/strsim-0.6.0.crate",
        type = "tar.gz",
        strip_prefix = "strsim-0.6.0",
        build_file = "//cargo:strsim-0.6.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__syn__0_10_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.10.8.crate",
        type = "tar.gz",
        strip_prefix = "syn-0.10.8",
        build_file = "//cargo:syn-0.10.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__syn__0_11_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.11.11.crate",
        type = "tar.gz",
        strip_prefix = "syn-0.11.11",
        build_file = "//cargo:syn-0.11.11.BUILD"
    )

    native.new_http_archive(
        name = "raze__synom__0_11_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/synom/synom-0.11.3.crate",
        type = "tar.gz",
        strip_prefix = "synom-0.11.3",
        build_file = "//cargo:synom-0.11.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__take__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/take/take-0.1.0.crate",
        type = "tar.gz",
        strip_prefix = "take-0.1.0",
        build_file = "//cargo:take-0.1.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__termion__1_5_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termion/termion-1.5.1.crate",
        type = "tar.gz",
        strip_prefix = "termion-1.5.1",
        build_file = "//cargo:termion-1.5.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__textwrap__0_9_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/textwrap/textwrap-0.9.0.crate",
        type = "tar.gz",
        strip_prefix = "textwrap-0.9.0",
        build_file = "//cargo:textwrap-0.9.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__thread_local__0_3_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-0.3.5.crate",
        type = "tar.gz",
        strip_prefix = "thread_local-0.3.5",
        build_file = "//cargo:thread_local-0.3.5.BUILD"
    )

    native.new_http_archive(
        name = "raze__tic__0_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tic/tic-0.3.0.crate",
        type = "tar.gz",
        strip_prefix = "tic-0.3.0",
        build_file = "//cargo:tic-0.3.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__time__0_1_38",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/time/time-0.1.38.crate",
        type = "tar.gz",
        strip_prefix = "time-0.1.38",
        build_file = "//cargo:time-0.1.38.BUILD"
    )

    native.new_http_archive(
        name = "raze__tiny_http__0_5_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tiny_http/tiny_http-0.5.8.crate",
        type = "tar.gz",
        strip_prefix = "tiny_http-0.5.8",
        build_file = "//cargo:tiny_http-0.5.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_core__0_1_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-core/tokio-core-0.1.10.crate",
        type = "tar.gz",
        strip_prefix = "tokio-core-0.1.10",
        build_file = "//cargo:tokio-core-0.1.10.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_io__0_1_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-io/tokio-io-0.1.4.crate",
        type = "tar.gz",
        strip_prefix = "tokio-io-0.1.4",
        build_file = "//cargo:tokio-io-0.1.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_proto__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-proto/tokio-proto-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "tokio-proto-0.1.1",
        build_file = "//cargo:tokio-proto-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__tokio_service__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tokio-service/tokio-service-0.1.0.crate",
        type = "tar.gz",
        strip_prefix = "tokio-service-0.1.0",
        build_file = "//cargo:tokio-service-0.1.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__tuple_utils__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tuple_utils/tuple_utils-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "tuple_utils-0.2.0",
        build_file = "//cargo:tuple_utils-0.2.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__unicase__2_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicase/unicase-2.1.0.crate",
        type = "tar.gz",
        strip_prefix = "unicase-2.1.0",
        build_file = "//cargo:unicase-2.1.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__unicode_width__0_1_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-width/unicode-width-0.1.4.crate",
        type = "tar.gz",
        strip_prefix = "unicode-width-0.1.4",
        build_file = "//cargo:unicode-width-0.1.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__unicode_xid__0_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-xid/unicode-xid-0.0.4.crate",
        type = "tar.gz",
        strip_prefix = "unicode-xid-0.0.4",
        build_file = "//cargo:unicode-xid-0.0.4.BUILD"
    )

    native.new_http_archive(
        name = "raze__unreachable__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unreachable/unreachable-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "unreachable-1.0.0",
        build_file = "//cargo:unreachable-1.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__url__0_2_38",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/url/url-0.2.38.crate",
        type = "tar.gz",
        strip_prefix = "url-0.2.38",
        build_file = "//cargo:url-0.2.38.BUILD"
    )

    native.new_http_archive(
        name = "raze__utf8_ranges__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/utf8-ranges/utf8-ranges-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "utf8-ranges-1.0.0",
        build_file = "//cargo:utf8-ranges-1.0.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__uuid__0_1_18",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/uuid/uuid-0.1.18.crate",
        type = "tar.gz",
        strip_prefix = "uuid-0.1.18",
        build_file = "//cargo:uuid-0.1.18.BUILD"
    )

    native.new_http_archive(
        name = "raze__vec_map__0_8_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/vec_map/vec_map-0.8.0.crate",
        type = "tar.gz",
        strip_prefix = "vec_map-0.8.0",
        build_file = "//cargo:vec_map-0.8.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__version_check__0_1_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/version_check/version_check-0.1.3.crate",
        type = "tar.gz",
        strip_prefix = "version_check-0.1.3",
        build_file = "//cargo:version_check-0.1.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__vk_sys__0_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/vk-sys/vk-sys-0.3.2.crate",
        type = "tar.gz",
        strip_prefix = "vk-sys-0.3.2",
        build_file = "//cargo:vk-sys-0.3.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__void__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/void/void-1.0.2.crate",
        type = "tar.gz",
        strip_prefix = "void-1.0.2",
        build_file = "//cargo:void-1.0.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__waterfall__0_7_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/waterfall/waterfall-0.7.1.crate",
        type = "tar.gz",
        strip_prefix = "waterfall-0.7.1",
        build_file = "//cargo:waterfall-0.7.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__which__1_0_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/which/which-1.0.3.crate",
        type = "tar.gz",
        strip_prefix = "which-1.0.3",
        build_file = "//cargo:which-1.0.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__winapi__0_2_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.2.8.crate",
        type = "tar.gz",
        strip_prefix = "winapi-0.2.8",
        build_file = "//cargo:winapi-0.2.8.BUILD"
    )

    native.new_http_archive(
        name = "raze__winapi__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.3.crate",
        type = "tar.gz",
        strip_prefix = "winapi-0.3.3",
        build_file = "//cargo:winapi-0.3.3.BUILD"
    )

    native.new_http_archive(
        name = "raze__winapi_build__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-build/winapi-build-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "winapi-build-0.1.1",
        build_file = "//cargo:winapi-build-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__winapi_i686_pc_windows_gnu__0_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.3.2.crate",
        type = "tar.gz",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.3.2",
        build_file = "//cargo:winapi-i686-pc-windows-gnu-0.3.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__winapi_x86_64_pc_windows_gnu__0_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.3.2.crate",
        type = "tar.gz",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.3.2",
        build_file = "//cargo:winapi-x86_64-pc-windows-gnu-0.3.2.BUILD"
    )

    native.new_http_archive(
        name = "raze__ws2_32_sys__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ws2_32-sys/ws2_32-sys-0.2.1.crate",
        type = "tar.gz",
        strip_prefix = "ws2_32-sys-0.2.1",
        build_file = "//cargo:ws2_32-sys-0.2.1.BUILD"
    )

    native.new_http_archive(
        name = "raze__zcfg__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/zcfg/zcfg-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "zcfg-0.2.0",
        build_file = "//cargo:zcfg-0.2.0.BUILD"
    )

    native.new_http_archive(
        name = "raze__zcfg_flag_parser__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/zcfg_flag_parser/zcfg_flag_parser-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "zcfg_flag_parser-0.2.0",
        build_file = "//cargo:zcfg_flag_parser-0.2.0.BUILD"
    )

