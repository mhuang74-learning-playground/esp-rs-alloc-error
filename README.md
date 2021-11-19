# esp-rs-alloc-error

Simple project to demonstrate strange errors with memory tests.

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

## Output

The logs show that only Memory Test #0 passes, the other two causes device to reboot trying to do a huge alloc.

```
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

```
B
B
B
A
A
A
A
A
A
A
A
A
A
A
A
```
