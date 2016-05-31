# Netpbm

A (mostly) conformant implementation of the [Netpbm](https://en.wikipedia.org/wiki/Netpbm)
project file formats in Rust.

# Goals
1. A reasonably conformant implementation of the four Netpbm file formats.
  * pbm
  * pgm
  * ppm
  * pam
2. Implement these in both regular (binary) and plain (ascii) forms.
3. Avoid any dependencies on either Rust crates or non-Rust code.

# Implemented Features

* Portable BitMap (pbm) files.
* Portable GrayMap (pgm) files.
* Portable PixMap (ppm) files.

# Features To Be Implemented

* Portable Arbitrary Map (pam) files.

# Intentionally Non-Conformant Features

These are parts of the package which don't conform to the standard and for which
there are no current plans to make conformant.

#### Multiple Images:
The standard allows for multiple images to be stored in the same file for some
formats. This implementation allows for only one image per file. It does not
check for the presence of multiple images and so the behavior is undefined.

#### Tools From The Main Netpbm Project:
This package is only interested in providing ways to load and save images using
the four Netpbm file formats. The actual Netpbm package has more than 300 tools
for doing work with image data. This is outside the scope of this particular
project.

#### Arbitrary bit depth:
The standard says that pgm files can specify any bit depth up to 65535 (the max
size for a u16). This library is set to read any value less than 256 as 255 and
any other value as 65535. This may mean that data saved under other bit depths
will be scaled incorrectly.
