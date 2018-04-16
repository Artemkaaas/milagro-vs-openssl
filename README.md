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

<table>  
  <th>v1.3.0</th>
  <th>v1.4.0</th>
  <tr>
    <td>
      <pre>
fn indy_prep_msg(command_handle: i32,
                 wallet_handle: i32,
                 sender_vk: *const c_char,
                 recipient_vk: *const c_char,
                 msg_data: *const u8,
                 msg_len: u32,
                 cb: Option<extern fn(command_handle_: i32,
                                      err: ErrorCode,
                                      encrypted_msg: *const u8,
                                      encrypted_len: u32)>)
        </pre>
    </td>
    <td>
      <pre>
fn indy_crypto_auth_crypt(command_handle: i32,
                          wallet_handle: i32,
                          my_vk: *const c_char,
                          their_vk: *const c_char,
                          msg_data: *const u8,
                          msg_len: u32,
                          cb: Option<extern fn(command_handle_: i32,
                                               err: ErrorCode,
                                               encrypted_msg: *const u8,
                                               encrypted_len: u32)>)
        </pre>
    </td>
  </tr>
</table>                                  
               
