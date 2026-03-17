#! /bin/env echo source-me

BUILD=(build --target-dir "./build/target")

RELEASE=(--release
	-Zpanic-immediate-abort
	-Zbuild-std=std,panic_abort)

DEBUG=()
