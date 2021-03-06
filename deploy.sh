# Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

#!/bin/bash

rev=$(git rev-parse --short HEAD)

cd book

git init
git config user.name "Igor.Shaposhnik"
git config user.email "shaposhnikigor95@bk.ru"
git remote add upstream "https://$GH_TOKEN@github.com/ruRust/rust-by-example-ru.git"
git fetch upstream && git reset upstream/gh-pages

#echo "rustbyexample.com" > CNAME

touch .

git add -A .

git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
