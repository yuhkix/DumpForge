# [Reverse Assembling Program Engineering(RAPE) Toolkit by xavo95](https://git.xeondev.com/xavo95/RAPE-toolkit)

This repository contains a subset of my private tools for Reverse Engineering, a new project(codename: Symphonic) will 
make an appearance soon. As such, all required dependencies are made public ahead of time.

Also on the past 6 months I received questions on: How do you find this? How do you get AES keys? How do you fix the 
dump? So I hope this helps people wanting to learn to

## Quick tool summary

- RAPE-WuWa (my addition utilizing the **PE Utils** and **AES Key Finder** library)
  - Dumps the Main AES Key from the given (unpacked) executable
- AES Key Finder
  - Should be self-explanatory but, basically after parsing a PE file you can pass image base, sections, and 
  data(raw binary) and the filter(this tool includes Restricted and Relax filters, but you can add more)
  - To get the 3 first params, please refer to PE Utils down below
- Cursor
  - An implementation for a cursor Read + Write. Rust already has one, but I needed something like string and 
  wide string parsing, so I created my own
- Offset Finder
  - This library allows to find patterns in executables
  - Allows to find either exact or partial matches by leveraging wildcards(??)
  - Also has options for silent reporting(skip_print_offset) or allow multiple matches
  - Leveraging PE Utils it returns both in file and RVA of the pattern found
- PE Utils
  - Subset of functions to work with PE files. Not very valuable alone, but it allows to omit repetitive code for the 
  rest of projects
- Restorer
  - Allows to go from a memory dump(Frida, and others(Including private dumpers)) to a file that has section table 
  fixed for more analysis with other tools
- TLZMA
  - Implements the LZMA algorithm use by a certain anti cheat software
