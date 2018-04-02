#! /usr/bin/env bash
set -e

bazel test //core/...  //experimental/...  //game/...

