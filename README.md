# egui webapp
    - CrossWebPlatformAssembly
---

## ToDo

* __Common__
    - [x] Debug log
    - [x] Separate View for Debug log
    - [ ] apply convenient traits and structs for wrapping **egui**
    - [ ] Rework App/View architecture

* __Sound__
    - [x] apply storage for SoundFonts into WASM (and binary)
    - [x] apply wrapper for **TinyAudio**
    - [x] apply connection trait between the wrappers
    - [x] apply default (Silent) and custom SoundRenders
    - [x] create **SimpleSynth** and apply wrapper for it
    - [x] apply wrapper for **RustySynth**

* __Applets__
    - [ ] basic View of base DoMiKkk excercise
    - [ ] test WASM on different platforms
    - [ ] move DoMiKkk to separate Application

---
> ### Powered by
> - egui: https://github.com/emilk/egui/
> - tinyaudio: https://github.com/mrDIMAS/tinyaudio/
> - rustysynth: https://github.com/sinshu/rustysynth/
