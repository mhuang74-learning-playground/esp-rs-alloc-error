# esp-rs-alloc-error

Simple project to demonstrate strange errors with memory tests.

All 3 tests do the same thing, namely, simulating writing 256 color Palette to in-memory buffer that's typical for encoding images. 

But Only Test #0 passes.

The other 2 fails with: 

```
memory allocation of 8388608 bytes failed
```

## Memory Tests

The 3 memory test types only differ on how buffer is being written to:

* Test #0: use Vec.push()

```
                for val in 0u8..255u8 {
                    my_vec.push(val);
                    my_vec.push(val);
                    my_vec.push(val);
                    my_vec.push(0);
                }
```

* Test #1: use Write.write_all()

```
                // turn Vector into Writable via Cursor
                let mut cursored  = std::io::Cursor::new(&mut my_vec);
                let writable_buffer: &mut dyn Write = &mut cursored;
                
                for val in 0u8..=255u8 {
                    writable_buffer.write_all(&[val, val, val, 0] )?;
                }
```

* Test #2: use Write.write_u8 from byteorder crate

```
                // turn Vector into Writable via Cursor
                let mut cursored  = std::io::Cursor::new(&mut my_vec);
                let writable_buffer: &mut dyn Write = &mut cursored;

                for val in 0u8..255u8 {
                    writable_buffer.write_u8(val)?;
                    writable_buffer.write_u8(val)?;
                    writable_buffer.write_u8(val)?;
                    writable_buffer.write_u8(0)?;
                }
```
## Environment

* esp-idf v4.3
* esp32-cam

## Stack Trace

decoded
```
0x400d604a: std::sys::unix::abort_internal at /home/mhuang/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/sys/unix/mod.rs:259
0x4008747e: panic_abort at /usr/local/projects/mhuang/3rd-party/esp-rs/esp-idf-sys-build-toolchain-shared/esp-idf-v4.3/components/esp_system/panic.c:356
0x40087c7d: esp_system_abort at /usr/local/projects/mhuang/3rd-party/esp-rs/esp-idf-sys-build-toolchain-shared/esp-idf-v4.3/components/esp_system/system_api.c:112
0x4008c94a: abort at /usr/local/projects/mhuang/3rd-party/esp-rs/esp-idf-sys-build-toolchain-shared/esp-idf-v4.3/components/newlib/abort.c:46
0x400d604a: std::sys::unix::abort_internal at /home/mhuang/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/sys/unix/mod.rs:259
0x400d5efa: std::process::abort at /home/mhuang/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/process.rs:1979
0x400d5f6d: rust_oom at /home/mhuang/.rustup/toolchains/esp/lib/rustlib/src/rust/library/std/src/alloc.rs:330
0x400de2a2: __rg_oom at /home/mhuang/.rustup/toolchains/esp/lib/rustlib/src/rust/library/alloc/src/alloc.rs:398
0x400de08a: alloc::alloc::handle_alloc_error at /home/mhuang/.rustup/toolchains/esp/lib/rustlib/src/rust/library/alloc/src/alloc.rs:367
0x400ddce0: alloc::raw_vec::handle_reserve at /home/mhuang/.rustup/toolchains/esp/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs:531
0x400ddce0: alloc::raw_vec::RawVec::reserve::do_reserve_and_handle at /home/mhuang/.rustup/toolchains/esp/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs:334
0x400de0f0: alloc::raw_vec::RawVec::reserve at /home/mhuang/.rustup/toolchains/esp/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs:338
0x400de0f0: alloc::vec::Vec::reserve at /home/mhuang/.rustup/toolchains/esp/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:805
0x400de0f0: alloc::vec::Vec::append_elements at /home/mhuang/.rustup/toolchains/esp/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1736
0x400de0f0: as alloc::vec::spec_extend::SpecExtend<&T,core::slice::iter::Iter>>::spec_extend at /home/mhuang/.rustup/toolchains/esp/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:85
0x400de15c: alloc::vec::Vec::extend_from_slice at /home/mhuang/.rustup/toolchains/esp/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2163
```

raw

```
abort() was called at PC 0x400d604a on core 0
Backtrace:0x4008747e:0x3ffb5b90 0x40087c7d:0x3ffb5bb0 0x4008c94a:0x3ffb5bd0 0x400d604a:0x3ffb5c40 0x400d5efa:0x3ffb5c60 0x400d5f6d:0x3ffb5c80 0x400de2a2:0x3ffb5ca0 0x400d2772 [__rust_alloc_error_handler:??:?]:0x3ffb5cc0 0x400de08a:0x3ffb5ce0 0x400ddce0:0x3ffb5d00 0x400de0f0:0x3ffb5d30 0x400de15c:0x3ffb5d58 |<-CORRUPTED
```


## Output

The logs show that only Memory Test #0 passes, the other two causes device to reboot trying to do a huge alloc.

```
I (155) psram: This chip is ESP32-D0WD
I (156) spiram: Found 64MBit SPI RAM device
I (156) spiram: SPI RAM mode: flash 40m sram 40m
I (159) spiram: PSRAM initialized, cache is in low/high (2-core) mode.
I (166) cpu_start: Pro cpu up.
I (170) cpu_start: Starting app cpu, entry point is 0x40080efc
I (0) cpu_start: App cpu up.
I (1058) spiram: SPI SRAM memory test OK
I (1066) cpu_start: Pro cpu start user code
I (1066) cpu_start: cpu freq: 160000000
I (1067) cpu_start: Application information:
I (1069) cpu_start: Project name:     libespidf
I (1075) cpu_start: App version:      1
I (1079) cpu_start: Compile time:     Nov 19 2021 13:35:48
I (1085) cpu_start: ELF file SHA256:  0000000000000000...
I (1091) cpu_start: ESP-IDF:          v4.3-dirty
I (1097) heap_init: Initializing. RAM available for dynamic allocation:
I (1104) heap_init: At 3FFAE6E0 len 00001920 (6 KiB): DRAM
I (1110) heap_init: At 3FFB2B50 len 0002D4B0 (181 KiB): DRAM
I (1117) heap_init: At 3FFE0440 len 00003AE0 (14 KiB): D/IRAM
I (1123) heap_init: At 3FFE4350 len 0001BCB0 (111 KiB): D/IRAM
I (1130) heap_init: At 4008D528 len 00012AD8 (74 KiB): IRAM
I (1136) spiram: Adding pool of 4096K of external SPI memory to heap allocator
I (1145) spi_flash: detected chip: generic
I (1149) spi_flash: flash io: dio
I (1154) cpu_start: Starting scheduler on PRO CPU.
I (0) cpu_start: Starting scheduler on APP CPU.
I (1163) spiram: Reserving pool of 32K of internal memory for DMA/internal allocations
Hello, world!
Running memory test #2
Running 2 KB memory test with WriteMethod::WriteU8...
2: allocated Vec<u8> of size: 2048
memory allocation of 8388608 bytes failed
abort() was called at PC 0x400d604a on core 0
Backtrace:0x4008700e:0x3ffb58b0 0x4008780d:0x3ffb58d0 0x4008c4d2:0x3ffb58f0 0x400d604a:0x3ffb5960 0x400d5efa:0x3ffb5980 0x400d5f6d:0x3ffb59a0 0x400de2a2:0x3ffb59c0 0x400d2772:0x3ffb59e0 0x400de08a:0x3ffb5a00 0x400ddce0:0x3ffb5a20 0x400de0f0:0x3ffb5a50 0x400de15c:0x3ffb5a78 |<-CORRUPTED
ELF file SHA256: 0000000000000000
Rebooting...
ets Jun  8 2016 00:22:57
rst:0xc (SW_CPU_RESET),boot:0x13 (SPI_FAST_FLASH_BOOT)
configsip: 0, SPIWP:0xee
clk_drv:0x00,q_drv:0x00,d_drv:0x00,cs0_drv:0x00,hd_drv:0x00,wp_drv:0x00
mode:DIO, clock div:2
load:0x3fff0048,len:12
ho 0 tail 12 room 4
load:0x3fff0054,len:4800
load:0x40078000,len:17448
load:0x4007c428,len:4840
entry 0x4007c6a0
I (156) psram: This chip is ESP32-D0WD
I (156) spiram: Found 64MBit SPI RAM device
I (156) spiram: SPI RAM mode: flash 40m sram 40m
I (159) spiram: PSRAM initialized, cache is in low/high (2-core) mode.
I (166) cpu_start: Pro cpu up.
I (170) cpu_start: Starting app cpu, entry point is 0x40080efc
I (161) cpu_start: App cpu up.
I (1058) spiram: SPI SRAM memory test OK
I (1067) cpu_start: Pro cpu start user code
I (1067) cpu_start: cpu freq: 160000000
I (1067) cpu_start: Application information:
I (1070) cpu_start: Project name:     libespidf
I (1075) cpu_start: App version:      1
I (1080) cpu_start: Compile time:     Nov 19 2021 13:35:48
I (1086) cpu_start: ELF file SHA256:  0000000000000000...
I (1092) cpu_start: ESP-IDF:          v4.3-dirty
I (1097) heap_init: Initializing. RAM available for dynamic allocation:
I (1104) heap_init: At 3FFAE6E0 len 00001920 (6 KiB): DRAM
I (1111) heap_init: At 3FFB2B50 len 0002D4B0 (181 KiB): DRAM
I (1117) heap_init: At 3FFE0440 len 00003AE0 (14 KiB): D/IRAM
I (1123) heap_init: At 3FFE4350 len 0001BCB0 (111 KiB): D/IRAM
I (1130) heap_init: At 4008D528 len 00012AD8 (74 KiB): IRAM
I (1136) spiram: Adding pool of 4096K of external SPI memory to heap allocator
I (1145) spi_flash: detected chip: generic
I (1149) spi_flash: flash io: dio
I (1154) cpu_start: Starting scheduler on PRO CPU.
I (0) cpu_start: Starting scheduler on APP CPU.
I (1164) spiram: Reserving pool of 32K of internal memory for DMA/internal allocations
Hello, world!
Running memory test #0
Running 2 KB memory test with WriteMethod::Direct...
2: allocated Vec<u8> of size: 2048
Allocated 2 KB blocks in step of 2 and wrote using method 'Direct'
Memory test #0 PASSED !
Restarting in 10 seconds...
Restarting in 9 seconds...
Restarting in 8 seconds...
Restarting in 7 seconds...
Restarting in 6 seconds...
Restarting in 5 seconds...
Restarting in 4 seconds...
Restarting in 3 seconds...
Restarting in 2 seconds...
Restarting in 1 seconds...
Restarting now.
ets Jun  8 2016 00:22:57
rst:0xc (SW_CPU_RESET),boot:0x13 (SPI_FAST_FLASH_BOOT)
configsip: 0, SPIWP:0xee
clk_drv:0x00,q_drv:0x00,d_drv:0x00,cs0_drv:0x00,hd_drv:0x00,wp_drv:0x00
mode:DIO, clock div:2
load:0x3fff0048,len:12
ho 0 tail 12 room 4
load:0x3fff0054,len:4800
load:0x40078000,len:17448
load:0x4007c428,len:4840
entry 0x4007c6a0
I (156) psram: This chip is ESP32-D0WD
I (156) spiram: Found 64MBit SPI RAM device
I (156) spiram: SPI RAM mode: flash 40m sram 40m
I (159) spiram: PSRAM initialized, cache is in low/high (2-core) mode.
I (166) cpu_start: Pro cpu up.
I (170) cpu_start: Starting app cpu, entry point is 0x40080efc
I (161) cpu_start: App cpu up.
I (1058) spiram: SPI SRAM memory test OK
I (1067) cpu_start: Pro cpu start user code
I (1067) cpu_start: cpu freq: 160000000
I (1067) cpu_start: Application information:
I (1070) cpu_start: Project name:     libespidf
I (1075) cpu_start: App version:      1
I (1080) cpu_start: Compile time:     Nov 19 2021 13:35:48
I (1086) cpu_start: ELF file SHA256:  0000000000000000...
I (1092) cpu_start: ESP-IDF:          v4.3-dirty
I (1097) heap_init: Initializing. RAM available for dynamic allocation:
I (1104) heap_init: At 3FFAE6E0 len 00001920 (6 KiB): DRAM
I (1111) heap_init: At 3FFB2B50 len 0002D4B0 (181 KiB): DRAM
I (1117) heap_init: At 3FFE0440 len 00003AE0 (14 KiB): D/IRAM
I (1123) heap_init: At 3FFE4350 len 0001BCB0 (111 KiB): D/IRAM
I (1130) heap_init: At 4008D528 len 00012AD8 (74 KiB): IRAM
I (1136) spiram: Adding pool of 4096K of external SPI memory to heap allocator
I (1145) spi_flash: detected chip: generic
I (1149) spi_flash: flash io: dio
I (1154) cpu_start: Starting scheduler on PRO CPU.
I (0) cpu_start: Starting scheduler on APP CPU.
I (1164) spiram: Reserving pool of 32K of internal memory for DMA/internal allocations
Hello, world!
Running memory test #2
Running 2 KB memory test with WriteMethod::WriteU8...
2: allocated Vec<u8> of size: 2048
memory allocation of 8388608 bytes failed
abort() was called at PC 0x400d604a on core 0
Backtrace:0x4008700e:0x3ffb58b0 0x4008780d:0x3ffb58d0 0x4008c4d2:0x3ffb58f0 0x400d604a:0x3ffb5960 0x400d5efa:0x3ffb5980 0x400d5f6d:0x3ffb59a0 0x400de2a2:0x3ffb59c0 0x400d2772:0x3ffb59e0 0x400de08a:0x3ffb5a00 0x400ddce0:0x3ffb5a20 0x400de0f0:0x3ffb5a50 0x400de15c:0x3ffb5a78 |<-CORRUPTED
ELF file SHA256: 0000000000000000
Rebooting...
ets Jun  8 2016 00:22:57
rst:0xc (SW_CPU_RESET),boot:0x13 (SPI_FAST_FLASH_BOOT)
configsip: 0, SPIWP:0xee
clk_drv:0x00,q_drv:0x00,d_drv:0x00,cs0_drv:0x00,hd_drv:0x00,wp_drv:0x00
mode:DIO, clock div:2
load:0x3fff0048,len:12
ho 0 tail 12 room 4
load:0x3fff0054,len:4800
load:0x40078000,len:17448
load:0x4007c428,len:4840
entry 0x4007c6a0
I (156) psram: This chip is ESP32-D0WD
I (156) spiram: Found 64MBit SPI RAM device
I (156) spiram: SPI RAM mode: flash 40m sram 40m
I (159) spiram: PSRAM initialized, cache is in low/high (2-core) mode.
I (166) cpu_start: Pro cpu up.
I (170) cpu_start: Starting app cpu, entry point is 0x40080efc
I (161) cpu_start: App cpu up.
I (1058) spiram: SPI SRAM memory test OK
I (1067) cpu_start: Pro cpu start user code
I (1067) cpu_start: cpu freq: 160000000
I (1067) cpu_start: Application information:
I (1070) cpu_start: Project name:     libespidf
I (1075) cpu_start: App version:      1
I (1080) cpu_start: Compile time:     Nov 19 2021 13:35:48
I (1086) cpu_start: ELF file SHA256:  0000000000000000...
I (1092) cpu_start: ESP-IDF:          v4.3-dirty
I (1097) heap_init: Initializing. RAM available for dynamic allocation:
I (1104) heap_init: At 3FFAE6E0 len 00001920 (6 KiB): DRAM
I (1111) heap_init: At 3FFB2B50 len 0002D4B0 (181 KiB): DRAM
I (1117) heap_init: At 3FFE0440 len 00003AE0 (14 KiB): D/IRAM
I (1123) heap_init: At 3FFE4350 len 0001BCB0 (111 KiB): D/IRAM
I (1130) heap_init: At 4008D528 len 00012AD8 (74 KiB): IRAM
I (1136) spiram: Adding pool of 4096K of external SPI memory to heap allocator
I (1145) spi_flash: detected chip: generic
I (1149) spi_flash: flash io: dio
I (1154) cpu_start: Starting scheduler on PRO CPU.
I (0) cpu_start: Starting scheduler on APP CPU.
I (1164) spiram: Reserving pool of 32K of internal memory for DMA/internal allocations
Hello, world!
Running memory test #1
Running 2 KB memory test with WriteMethod::Write...
2: allocated Vec<u8> of size: 2048
memory allocation of 8388608 bytes failed
abort() was called at PC 0x400d604a on core 0
Backtrace:0x4008700e:0x3ffb58b0 0x4008780d:0x3ffb58d0 0x4008c4d2:0x3ffb58f0 0x400d604a:0x3ffb5960 0x400d5efa:0x3ffb5980 0x400d5f6d:0x3ffb59a0 0x400de2a2:0x3ffb59c0 0x400d2772:0x3ffb59e0 0x400de08a:0x3ffb5a00 0x400ddce0:0x3ffb5a20 0x400de0f0:0x3ffb5a50 0x400de15c:0x3ffb5a78 |<-CORRUPTED
ELF file SHA256: 0000000000000000
Rebooting...

```
