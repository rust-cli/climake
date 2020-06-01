"""Converts all given text into test with `/// ` or `//! ` beginning on each line"""

import os
from pathlib import Path

IN_OUT_PATH = Path(os.path.dirname(os.path.abspath(__file__))) / "text.rs"

if int(input("0: /// \n1: //!\nPrefix choice [0-1]: ")) == 0:
    prefix = "///"
else:
    prefix = "//!"

input(f"Add your text to `{IN_OUT_PATH}` and press [ENTER]: ")

output = ""

with open(IN_OUT_PATH, "r") as f_in:
    for line in f_in.readlines():
        if len(line) == 1:
            output += f"{prefix}{line}" # no space on empty line
        else:
            print(line)
            output += f"{prefix} {line}"

with open(IN_OUT_PATH, "w+") as f_out:
    f_out.write(output)

print(f"Saved to `{IN_OUT_PATH}`, same place as you entered text.")
