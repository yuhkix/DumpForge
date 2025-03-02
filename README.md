# DumpForge

> **Credits:** Full credit goes to [xavo95](https://git.xeondev.com/xavo95/RAPE-toolkit) for the invaluable  
> **Reverse Assembling Program Engineering (RAPE) Toolkit**.  
> Feel free to submit **pull requests** to update or add features.

---

## 📌 Quick Tool Summary

### 🔹 DumpForge
- Utilizes **PE Utils**, **AES Key Finder** and **Restorer** libraries.
- Dumps the **Main AES key** of an **UE Game**
- Restores **section headers** from memory dumps
- Fetches the specified executables **Imports** _(experimental)_

### 🔹 AES Key Finder
- Parses a **PE file** and extracts **AES keys** based on provided parameters.
- Requires:
  - **Image Base**
  - **Sections**
  - **Raw Binary Data**
  - **Filter Type** (Restricted/Relaxed; customizable)
- For obtaining the required parameters, refer to **PE Utils** below.

### 🔹 Offset Finder
- Searches for **patterns in executables**.
- Supports:
  - **Exact or partial matches** (via wildcards `??`).
  - **Silent reporting** (`skip_print_offset`).
  - **Multiple match handling**.
- Works with **PE Utils** to return both **file offsets** and **RVA** (Relative Virtual Address).

### 🔹 Restorer
- Converts **memory dumps** (Frida and other dumpers, including private ones)  
  into a **reconstructed PE file**.
- Fixes the **section table** to allow further analysis with other tools.

### 🔹 PE Utils (by xavo95)
- A collection of **PE file handling functions**.
- Simplifies **repetitive PE-related tasks** for other projects.
- Useful for extracting necessary **PE metadata**.
