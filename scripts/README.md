# `/scripts`

This directory contains some misc development and build scripts for easier development in climake. These are not distributed with the build and are here for the express purpose of development.

Try to stick with python or rust here, ensuring that any rust is completely seperate from the main `climake` module and is lightweight (very little dependancies). If it is larger than that, it's easier just to make your own crate/package to preform the task instead of including it here.
