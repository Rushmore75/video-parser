## Video Parser
This takes a visual media input (photo / video / etc) and uses ffmpeg to read the rgb values then writes them to `stdout` as raw data. This can the be piped to [/dev/ttyACMx](https://github.com/Rushmore75/led_matrix) which will display them.
To find the arduino you can run:
```
ls /dev/serial/by-id/ | grep arduino | xargs -I{} readlink /dev/serial/by-id/{}
```
which will output `../../ttyACMX` or simmilar. Then `/dev/ttyACMX` (for example) is your arduino.
From which you can run `cargo run > /dev/ttyACMX` to pipe the parsed video into it.
