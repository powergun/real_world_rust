#!/usr/bin/env python

import os
import sys
import shutil
import tempfile


def main(src, dest):
    if os.path.isdir(dest) and not os.path.islink(dest):
        shutil.rmtree(dest, True)
    elif os.path.exists(dest):
        os.remove(dest)
    os.symlink(src, dest)
    print(dest)


if __name__ == '__main__':
    src = os.path.dirname(os.path.abspath(sys.argv[0]))
    main(
        os.path.join(src, 'testdata'), 
        os.path.join(tempfile.gettempdir(), 'rw_rust_testdata')
    )