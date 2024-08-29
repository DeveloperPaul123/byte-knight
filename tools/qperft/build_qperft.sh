/*
 * Filename: f:\Repositories\byte-knight\tools\qperft\build_qperft.sh
 * Path: f:\Repositories\byte-knight\tools\qperft
 * Created Date: Thursday, August 29th 2024, 1:48:29 pm
 * Author: Paul Tsouchlos (DeveloperPaul123)
 * 
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 */
#!/bin/bash

if [ -f "build/qperft.exe" ]; then
    rm build/qperft.exe
fi
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DCMAKE_C_COMPILER=clang -G "Ninja"
cmake --build build

# End of script
