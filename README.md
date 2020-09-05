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

### Using a LADSPA plugin
Then, create the new device using:
```sh
pactl load-module module-null-sink sink_name=nonoise_mic_denoised_out rate=48000

pactl load-module module-ladspa-sink sink_name=nonoise_mic_raw_in master=nonoise_mic_denoised_out label=noise_suppressor_mono plugin=/home/brendan/development/projects/nonoise/librnnoise-048783866.so control=95

pactl load-module module-loopback source=alsa_input.usb-Microsoft_Microsoft___LifeCam_HD-3000-02.mono-fallback sink=nonoise_mic_raw_in channels=1 latency_msec=1
 
pactl load-module module-remap-source master=nonoise_mic_denoised_out.monitor source_name=nonoise_mic_remap source_properties="device.description='NoNoiseMicrophone'"
```

### Recording and playing back. Currently needed for running the virtual mic
```sh
pactl load-module module-null-sink sink_name=nonoise_mic_denoised_out rate=48000
pactl load-module module-remap-source master=nonoise_mic_denoised_out.monitor source_name=nonoise_mic_remap source_properties="device.description='NoNoiseMicrophone'"
```

## Resources
- http://adventures.michaelfbryan.com/posts/audio-processing-for-dummies/
- https://docs.rs/rsynth/0.1.1/rsynth/
- https://github.com/mitchmindtree
- https://nannou.cc/posts/moss_grant_completion