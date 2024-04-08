# led_game
## Installation instructions
* `git clone https://github.com/u007d/led_game`

OR

* cargo generate --git https::/github.com/u007d/embassy-template --name led_rs --force
* ONLY if you are using `elf2uf2-rs` to flash your board instead of a debugger probe:
  * change `[target.'cfg(all(target_arch = "arm", target_os = "none"))']` section to match below:
      ```toml
      [target.'cfg(all(target_arch = "arm", target_os = "none"))']
      #runner = "probe-rs run --chip RP2040"
      runner = "elf2uf2-rs -ds target/thumbv6m-none-eabi/debug/led_game"
      ```