# rape-wuwa
> All Credits go to [xavo95](https://git.xeondev.com/xavo95/RAPE-toolkit) for the helpful reverse assembling program engineering(rape) toolkit

## Quick tool summary

- RAPE-WuWa (my addition utilizing the **PE Utils** and **AES Key Finder** library)
  - Currently only dumps the main aes key, but i do plan to add support for pak 26-28 whenever i find a way to automize the necessary steps
- AES Key Finder (by xavo)
  - Should be self-explanatory but, basically after parsing a PE file you can pass image base, sections, and 
  data(raw binary) and the filter(this tool includes Restricted and Relax filters, but you can add more)
  - To get the 3 first params, please refer to PE Utils down below
- Offset Finder
  - This library allows to find patterns in executables
  - Allows to find either exact or partial matches by leveraging wildcards(??)
  - Also has options for silent reporting(skip_print_offset) or allow multiple matches
  - Leveraging PE Utils it returns both in file and RVA of the pattern found
- PE Utils (by xavo)
  - Subset of functions to work with PE files. Not very valuable alone, but it allows to omit repetitive code for the 
  rest of projects
