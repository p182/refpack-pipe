Simple CLI wrapper for [refpack-rs](https://github.com/actioninja/refpack-rs) library to compress files using stdin/stdout pipe

## License Notices for Dependencies

- **[refpack-rs]**  
  License: Mozilla Public License 2.0 (MPL 2.0)  
  Authors: https://github.com/actioninja  
  Link: https://crates.io/crates/refpack

---

### Notes

- This project acts as a wrapper around the above MPL-licensed library without modifying its source code.  
- The MPL-licensed library is distributed according to the terms of MPL 2.0.  
- Your own code is licensed separately under the chosen license.  
- All copyright and license notices for dependencies are preserved as required.







It was made to use with projects in another languages (eg. Python, etc.), but you can use in any way you want


You can build it by installing Rust toolchain and running second command to build

**Rust Install on desktop OS:**

https://www.rust-lang.org/tools/install

**Rust Install on Termux (Android):**

``pkg install rust``

**Build in any OS:**

``cargo build --release``

**If you're using Windows you can download prebuilt .exe (default compression mode, not optimal) via GitHub Actions:**

https://github.com/p182/refpack-pipe/actions/runs/16153722687 (artifact download at the bottom of page)

https://github.com/p182/refpack-pipe/actions/runs/16153722687/artifacts/3490015212 (direct link to built zipped .exe, don't forget to unpack it, of course)
