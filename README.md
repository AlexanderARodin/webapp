# egui webapp
    - CrossWebPlatformAssembly
---

## ToDo

* __Common__
    - [x] Debug log
    - [x] Separate View for Debag log
    - [ ] apply convenient traits and structs for wraping **egui**
    - [ ] Rework App/View architecture

* __Sound__
    - [x] apply storage for SoundFonts into WASM
    - [x] apply wraper for **TinyAudio**
    - [x] apply connection trait between the wrapers
    - [x] apply default (Silent) and custom SoundRenders
    - [x] create **SimpleSynth** and apply wraper for it
    - [ ] apply wraper for **RustySynth**

* __Applets__
    - [ ] basic View of base DoMiKkk excercise
    - [ ] test WASM on different platforms
    - [ ] move DoMiKkk to separate Application

---
> ### Powered by
> - egui: https://github.com/emilk/egui/
> - tinyaudio: https://github.com/mrDIMAS/tinyaudio/
> - rustysynth: https://github.com/sinshu/rustysynth/
