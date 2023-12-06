# boilit

boilit is a configurable cli tool to help you quickly create files passed on patterns of your project

## Getting started

### Via Crates.io

```shell
cargo install boilit
```

### Via GitHub Release

Download the correct file depending on your OS.

Create a `boilit.toml` file with your custom file pattern definitions.

Once created you can define your file names, contents, and other parameters into your config

## CLI

Use `boilit --help` to view available commands at any point

note: env -u IPHONEOS_DEPLOYMENT_TARGET -u TVOS_DEPLOYMENT_TARGET LC_ALL="C" PATH="/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:/snap/bin:/home/runner/.local/bin:/opt/pipx_bin:/home/runner/.cargo/bin:/home/runner/.config/composer/vendor/bin:/usr/local/.ghcup/bin:/home/runner/.dotnet/tools:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin" VSLANG="1033" ZERO_AR_DATE="1" "cc" "-arch" "x86_64" "-m64" "/tmp/rustcV5iq17/symbols.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.00.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.01.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.02.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.03.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.04.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.05.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.06.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.07.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.08.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.09.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.10.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.11.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.12.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.13.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.14.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.boilit.d6b366db0e5ccac6-cgu.15.rcgu.o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca.32z9ougpgz4berm6.rcgu.o" "-L" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps" "-L" "/home/runner/work/boilit/boilit/target/release/deps" "-L" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libtoml-c88e24007e5cb37e.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libserde-3d316c5e5b3e6097.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libcolored-13cf181b0f8f680c.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/liblazy_static-88f9999c765d9a3a.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libis_terminal-34e8686ccc6c838b.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/librustix-a5e97ce0974fd437.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libbitflags-93bb810cb56937b0.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/liberrno-e3809ba1920fa9aa.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/liblibc-77e17d77ccfef671.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libclap-a8c089facae60f78.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libclap_builder-2cda153b56c9e931.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libstrsim-71f0bd49dced1682.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libanstream-59ad970c5c810f07.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libanstyle_query-634d4613d9fa4da1.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libcolorchoice-93784ee6ad97cf72.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libanstyle_parse-f12ac334025aa3b9.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libutf8parse-de899f296877eba5.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libclap_lex-dbb13f8b51b233ba.rlib" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/libanstyle-0f02764cdb2f14f3.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libstd-0a295fcceaea49ca.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-3a3d571cea166ad9.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libobject-6b5831e6a9f3395e.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libmemchr-780e041d20a8486f.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libaddr2line-6ff591d5d0f5280b.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libgimli-f6e4539f49dd4d64.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-99bb440263492792.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libstd_detect-95028f1972548dce.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-d728baeb137ed657.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-a57a8b2bc725f9db.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libminiz_oxide-15260feb63d79227.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libadler-6fb16cb10b84eb26.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libunwind-dce5839c1ace228d.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-2440211cb85a82f6.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/liblibc-2085d7e7ee4d73e5.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/liballoc-e4d534c5c6eaccae.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-0f1f312e3dd9e4c9.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libcore-e79288f2566c0676.rlib" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-b000c82e0aa2127d.rlib" "-liconv" "-lSystem" "-lc" "-lm" "-L" "/home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-apple-darwin/lib" "-o" "/home/runner/work/boilit/boilit/target/x86_64-apple-darwin/release/deps/boilit-ce4c5c7947ffe5ca" "-Wl,-dead_strip" "-nodefaultlibs"

## Releasing

1. Bump the version in Cargo.toml
2. Create a new release via GitHub
3. GHA will create the windows and linux binary/exe
4. Run buildMacOS.sh on a mac
5. Upload the tar.gz to the release page
