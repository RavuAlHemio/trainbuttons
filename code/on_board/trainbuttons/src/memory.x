/*
  STM32G0B0KET
  STM32:  STM32 architecture
  G0:     Cortex-M0+, low-power, wide selection of variants
  B:      144k RAM (7 = 36k, 5 = 18k, 3 = 8k)
  0:      base model
  K:      32-pin (J = 8-pin, F = 20-pin, C = 48-pin, R = 64-pin, V = 100-pin)
  E:      512k flash (B = 128k, 8 = 64k, 6 = 32k)
  T:      LQFP (M = SO, P = TSSOP)
*/
MEMORY
{
  /* NOTE K = KiB = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 144K
}
