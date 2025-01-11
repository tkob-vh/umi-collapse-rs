#!/bin/bash

## Need sudo permission.
## noswap option is supported only after v6.4
mount -t tmpfs -o size=100g,huge=advise,mpol=default tmpfs /m5C-tmp
