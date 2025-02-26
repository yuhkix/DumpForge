# RAPE-WuWa
> All Credits go to [xavo95](https://git.xeondev.com/xavo95/RAPE-toolkit) for the helpful rape-toolkit

## Quick tool summary

- RAPE-WuWa (my addition utilizing the **PE Utils** and **AES Key Finder** library)
  - Dumps the Main AES Key from the given (unpacked) executable
- AES Key Finder (by xavo)
  - Should be self-explanatory but, basically after parsing a PE file you can pass image base, sections, and 
  data(raw binary) and the filter(this tool includes Restricted and Relax filters, but you can add more)
  - To get the 3 first params, please refer to PE Utils down below
- PE Utils (by xavo)
  - Subset of functions to work with PE files. Not very valuable alone, but it allows to omit repetitive code for the 
  rest of projects
