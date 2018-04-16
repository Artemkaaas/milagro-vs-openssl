# Libindy migration Guide

There are a lot APIs that have been changed in Libindy 1.4.0.
This document is written for developers using Libindy 1.3.0 to provide necessary information 
to simplify transition and to introduce new features.

In the following tables for each API part, there are mappings of how 1.3.0 functionality maps to 1.4.0. 
Functions from v1.3.0 are listed in the left column, and the equivalent 1.4.0 function are placed in the right column. 
Some function can take different parameters or return different data.
if there is not changes for some function, the will be placed word 'same' in the right column.

### Agent API mapping
The Agent API was completely deleted from Libindy API but his functionality was simplified and saved as part of Crypto API.

v1.3.0 | v.1.4.0
------------ | -------------
```indy_prep_msg``` | ```indy_crypto_auth_crypt```
```indy_prep_anonymous_msg``` | ```indy_crypto_anon_crypt```
```indy_parse_msg``` | <ul><li>item1</li><li>item2</li></ul>
                                    
                
