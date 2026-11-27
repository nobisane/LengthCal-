# lengthcal

A length converter written in Rust (egui). Tap any box, type on the number pad,
and all other boxes update.

Units: km, m, dm, cm, mm, µm, nm, mile, yard, foot, inch, nautical mile.

## Build the APK on GitHub (nothing to install)
1. Create a new GitHub repository.
2. Upload the *contents* of this folder to it, so Cargo.toml is at the top level.
   Include the hidden `.github` folder. If the web upload skips it, use
   "Add file > Create new file", name it `.github/workflows/build.yml`,
   and paste in the file's contents.
3. Open the **Actions** tab, choose **Build APK**, then **Run workflow**
   (it may already have started from your upload).
4. When it finishes, download **lengthcal-apk**, unzip it, copy `lengthcal.apk`
   to your phone and open it (allow "install unknown apps").

## Build on your own computer
Needs Rust, a JDK, the Android SDK (platform 34, build-tools) and the NDK.
Set `ANDROID_HOME` and `ANDROID_NDK_ROOT`, then:

    rustup target add aarch64-linux-android armv7-linux-androideabi
    cargo install cargo-apk --version 0.10.0 --locked
    cargo apk build --lib

Output: `target/debug/apk/lengthcal.apk`

## Notes
- The APK is signed with a debug key: fine for your own phone, not for the Play Store.
  Each fresh GitHub build may use a new key, so uninstall the old app before updating.
- cargo-apk is no longer actively developed (its successor is xbuild).
- Tests: `cargo test`. The conversion code is in `src/convert.rs`.
