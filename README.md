# Libindy migration Guide

There are a lot APIs that have been changed in Libindy 1.4.0.
This document is written for developers using Libindy 1.3.0 to provide necessary information 
to simplify transition and to introduce new features.

In the following tables for each API part, there are mappings of how 1.3.0 functionality maps to 1.4.0. 
Functions from v1.3.0 are listed in the left column, and the equivalent 1.4.0 function are placed in the right column. 
Some function can take different parameters or return different data.
if there is not changes for some function, the will be placed word 'same' in the right column.

### Agent API mapping
The Agent API was completely deleted from Libindy but his functionality was simplified and saved as part of Crypto API.

<table>  
  <th>v1.3.0 - Agent API</th>
  <th>v1.4.0 - Crypto API</th>
  <tr>
    <td>
      <pre>
indy_prep_msg(command_handle: i32,
              wallet_handle: i32,
              sender_vk: *const c_char,
              recipient_vk: *const c_char,
              msg_data: *const u8,
              msg_len: u32,
              cb: fn(command_handle_: i32,
                     err: ErrorCode,
                     encrypted_msg: *const u8,
                     encrypted_len: u32))
              </pre>
    </td>
    <td>
      <pre>
indy_crypto_auth_crypt(command_handle: i32,
                       wallet_handle: i32,
                       my_vk: *const c_char,
                       their_vk: *const c_char,
                       msg_data: *const u8,
                       msg_len: u32,
                       cb: fn(command_handle_: i32,
                              err: ErrorCode,
                              encrypted_msg: *const u8,
                              encrypted_len: u32))
        </pre>
    </td>
  </tr>
  <tr>
    <td>
      <pre>
indy_prep_anonymous_msg(
          command_handle: i32,
          recipient_vk: *const c_char,
          msg_data: *const u8,
          msg_len: u32,
          cb: fn(command_handle_: i32,
                 err: ErrorCode,
                 encrypted_msg: *const u8,
                 encrypted_len: u32))
        </pre>
    </td>
    <td>
      <pre>
indy_crypto_anon_crypt(
          command_handle: i32,
          their_vk: *const c_char,
          msg_data: *const u8,
          msg_len: u32,
          cb: fn(command_handle_: i32,
                 err: ErrorCode,
                 encrypted_msg: *const u8,
                 encrypted_len: u32))
        </pre>
    </td>
  </tr>
<tr>
    <td rowspan="2">
      <pre>
indy_parse_msg(command_handle: i32,
               wallet_handle: i32,
               recipient_vk: *const c_char,
               encrypted_msg: *const u8,
               encrypted_len: u32,
               cb: fn(command_handle_: i32,
                      err: ErrorCode,
                      sender_vk: *const c_char,
                      msg_data: *const u8,
                      msg_len: u32))
      </pre>
    </td>
    <td>
      <pre>
indy_crypto_auth_decrypt(command_handle: i32,
                         wallet_handle: i32,
                         my_vk: *const c_char,
                         encrypted_msg: *const u8,
                         encrypted_len: u32,
                         cb: fn(command_handle_: i32,
                                err: ErrorCode,
                                their_vk: *const c_char,
                                msg_data: *const u8,
                                msg_len: u32))
      </pre>
    </td>
  </tr>
  <tr>
        <td>
      <pre>
indy_crypto_anon_decrypt(command_handle: i32,
                         wallet_handle: i32,
                         my_vk: *const c_char,
                         encrypted_msg: *const u8,
                         encrypted_len: u32,
                         cb: fn(command_handle_: i32,
                                err: ErrorCode,
                                msg_data: *const u8,
                                msg_len: u32))
      </pre>
    </td>
  </tr>
</table>                                  
               
