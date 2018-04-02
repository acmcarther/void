#! /usr/bin/env bash
set -e

bazel build //core/...  //experimental/...  //game/...

