# thonkfan

Config is inspired by thinkfan.

Example config:

```toml
# /etc/thonkfan.toml

# CPU temp sensor
thermal = '/sys/devices/platform/thinkpad_hwmon/hwmon/hwmon4/temp1_input'

# thinkpad_acpi fan interface
fan = '/proc/acpi/ibm/fan'

#  [LEVEL, LOW, HIGH]
#  LEVEL is the fan level to use (0-7 with thinkpad_acpi)
#  LOW is the temperature at which to step down to the previous level
#  HIGH is the temperature at which to step up to the next level
#  All numbers are integers.
curve = [
  [0, 0,  55],
  [1, 48, 60],
  [2, 50, 61],
  [3, 52, 63],
  [4, 56, 65],
  [5, 59, 66],
  [7, 63, 32767],
]
```
