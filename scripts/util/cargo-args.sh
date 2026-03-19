#! /bin/env echo source-me

BUILD=(build --target-dir "./build/target")

RELEASE_ARGS=(--release
	-Zpanic-immediate-abort
	-Zbuild-std=std,panic_abort)

DEBUG_ARGS=()
