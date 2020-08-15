## Pulse Audio 
https://github.com/toadjaune/pulseaudio-config

## LADSPA
https://github.com/werman/noise-suppression-for-voice

## Notes
- `DenoiseState::process_frame` probably returns `vadProbability` based on [noise-suppression-for-voice](https://github.com/werman/noise-suppression-for-voice/blob/34003bd9ab1509708eab61ef458feaae23327495/src/common/src/RnNoiseCommonPlugin.cpp#L39)

For example, to create a new mono device with noise-reduced audio from your microphone, first, find your mic name using e.g.:
```sh
pactl list sources short
```

Then, create the new device using:
```sh
pacmd load-module module-null-sink sink_name=nnnoiseless_mic_out rate=48000

pacmd load-module module-ladspa-sink sink_name=nnnoiseless_mic_raw_in sink_master=nnnoiseless_mic_out label=noiseless_suppressor_mono plugin=/home/brendan/development/projects/nonoise/target/debug/libnnnoiseless_ladspa.so control=95

pacmd load-module module-loopback source=alsa_input.usb-Microsoft_Microsoft___LifeCam_HD-3000-02.mono-fallback sink=nnnoiseless_mic_raw_in channels=1

pacmd load-module module-remap-source master=nnnoiseless_mic_out.monitor source_name=nnnoiseless_mic_remap source_properties="device.description='NNNoiseless Microphone'"
```