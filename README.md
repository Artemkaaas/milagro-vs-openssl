# Libindy migration Guide

## A Developer Guide for Libindy migration

There are a lot APIs that have been changed in Libindy 1.4.0.
This document is written for developers using Libindy 1.3.0 to provide necessary information 
to simplify transition and to introduce to new features of Libindy 1.4.0.

In the following tables for each Libindy API part, there are mappings of how 1.3.0 functionality maps to 1.4.0. 
Functions from v1.3.0 are listed in the left column, and the equivalent 1.4.0 function are placed in the right column. 
Some function were whether added or deleted or can take different parameters or return different data.
If there is not any changes for some function, the symbol '=' will be placed in the right column.

* [Signus API](#signus-api-mapping)
* [Crypto API](#crypto-api-mapping)
* [Agent API](#agent-api-mapping)

### Signus API mapping
The most significant change of this part is renaming Signus API to Did API. 
Furthermore, some functions of Signus API were deleted because the same goals can be achieved by using a combination of others.

<table>  
  <th>v1.3.0 - Signus API</th>
  <th>v1.4.0 - Crypto API</th>
   <tr> 
     <th colspan="2">
       Signs a message
     </th>
   </tr>
   <tr>
     <td>
       <pre>
indy_sign(...)
               </pre>
     </td>
     <td>
       <b>DELETED</b> <span>(use combination of either <i>did.indy_key_for_did</i> or <i>did.indy_key_for_local_did</i> with <i>crypto.indy_crypto_sign</i> instead)</span>
     </td>
   </tr>
   <tr> 
     <th colspan="2">
         Verify a signature
     </th>
   </tr>
   <tr>
     <td>
       <pre>
indy_verify_signature(...)
               </pre>
     </td>
     <td>
       <b>DELETED</b> <span>(use combination of either <i>did.indy_key_for_did</i> or <i>did.indy_key_for_local_did</i> with <i>crypto.indy_crypto_verify</i> instead)</span>
     </td>
   </tr>
   <tr> 
     <th colspan="2">
         Encrypts a message
     </th>
   </tr>
   <tr>
     <td>
       <pre>
indy_encrypt(...)
               </pre>
     </td>
     <td>
       <b>DELETED</b> <span>(use combination of either <i>did.indy_key_for_did</i> or <i>did.indy_key_for_local_did</i> with <i>crypto.indy_crypto_auth_crypt</i> instead)</span>
     </td>
   </tr>
   <tr> 
     <th colspan="2">
         Decrypts a message
     </th>
   </tr>
   <tr>
     <td>
       <pre>
indy_decrypt(...)
               </pre>
     </td>
     <td>
       <b>DELETED</b> <span>(use combination of either <i>did.indy_key_for_did</i> or <i>did.indy_key_for_local_did</i> with <i>crypto.indy_crypto_auth_decrypt</i> instead)</span>
     </td>
   </tr>
   <tr> 
     <th colspan="2">
         Encrypts a message by anonymous-encryption scheme
     </th>
   </tr>
   <tr>
     <td>
       <pre>
indy_encrypt_sealed(...)
               </pre>
     </td>
     <td>
       <b>DELETED</b> <span>(use combination of either <i>did.indy_key_for_did</i> or <i>did.indy_key_for_local_did</i> with <i>crypto.indy_crypto_anon_crypt</i> instead)</span>
     </td>
   </tr>
   <tr> 
     <th colspan="2">
         Decrypts a message by anonymous-encryption scheme
     </th>
   </tr>
   <tr>
     <td>
       <pre>
indy_decrypt_sealed(...)
               </pre>
     </td>
     <td>
       <b>DELETED</b> <span>(use combination of either <i>did.indy_key_for_did</i> or <i>did.indy_key_for_local_did</i> with <i>crypto.indy_crypto_anon_decrypt</i> instead)</span>
     </td>
   </tr>
   <tr> 
     <th colspan="2">
         Get info about My DID in format: DID, verkey, metadata
     </th>
   </tr>
   <tr>
     <td>
       <b>NEW</b>
     </td>
     <td>
       <pre>
indy_get_my_did_with_meta(command_handle: i32,
                          wallet_handle: i32,
                          my_did: *const c_char,
                          cb: fn(xcommand_handle: i32,
                                 err: ErrorCode,
                                 did_with_meta: *const c_char))
               </pre>
     </td>
   </tr>
   <tr> 
     <th colspan="2">
         Lists created DIDs with metadata as JSON array with each DID in format: DID, verkey, metadata
     </th>
   </tr>
   <tr>
     <td>
       <b>NEW</b>
     </td>
     <td>
       <pre>
indy_list_my_dids_with_meta(command_handle: i32,
                            wallet_handle: i32,
                            cb: fn(xcommand_handle: i32, 
                                   err: ErrorCode,
                                   ids: *const c_char))
               </pre>
     </td>
   </tr>
   <tr> 
     <th colspan="2">
        Retrieves abbreviated verkey if it is possible otherwise return full verkey.
     </th>
   </tr>
   <tr>
     <td>
       <b>NEW</b>
     </td>
     <td>
       <pre>
indy_abbreviate_verkey(command_handle: i32,
                       did: *const c_char,
                       full_verkey: *const c_char,
                       cb: fn(xcommand_handle: i32, 
                              err: ErrorCode,
                              verkey: *const c_char))
               </pre>
     </td>
   </tr>
  <tr> 
    <th colspan="2">
      Creates key for a new DID
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_create_and_store_my_did(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
      Generated temporary key for an existing DID.
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_replace_keys_start(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        Apply temporary key as main for an existing DID 
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_replace_keys_apply(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        Saves their DID for a pairwise connection in a secured Wallet
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_store_their_did(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        Returns ver key (key id) for the given DID.
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_key_for_did(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  </tr>
  <tr> 
    <th colspan="2">
        Returns ver key (key id) for the given DID.
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_key_for_local_did(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        Set/replace endpoint information for the given DID.
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_set_endpoint_for_did(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        Gets endpoint information for the given DID.
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_get_endpoint_for_did(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        Saves/replaces the meta information for the giving DID in the wallet.
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_set_did_metadata(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        Retrieves the meta information for the giving DID in the wallet.    
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_get_did_metadata(...)
      </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
   <tr> 
     <th colspan="2">
         Signs a message by a signing key associated with my DID
     </th>
   </tr>
</table> 
    
### Crypto API mapping

<table>  
  <th>v1.3.0 - Crypto API</th>
  <th>v1.4.0 - Crypto API</th>
  <tr> 
    <th colspan="2">
       Encrypt a message by authenticated-encryption scheme.
    </th>
  </tr>
  <tr>
    <td>
<pre>
indy_crypto_box(command_handle: i32,
                wallet_handle: i32,
                my_vk: *const c_char,
                their_vk: *const c_char,
                message_raw: *const u8,
                message_len: u32,
                cb: fn(xcommand_handle: i32, 
                       err: ErrorCode,
                       encrypted_msg_raw: *const u8, 
                       encrypted_msg_len: u32,
                       nonce_raw: *const u8, 
                       nonce_len: u32))
        </pre>
    </td>
    <td>
<pre>
indy_crypto_auth_crypt(
                command_handle: i32,
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
    <th colspan="2">
       Decrypt a message by authenticated-encryption scheme.
    </th>
  </tr>
  <tr>
    <td>
<pre>
indy_crypto_box_open(
                command_handle: i32,
                wallet_handle: i32,
                my_vk: *const c_char,
                their_vk: *const c_char,
                encrypted_msg_raw: *const u8,
                encrypted_msg_len: u32,
                nonce_raw: *const u8,
                nonce_len: u32,
                cb: fn(xcommand_handle: i32, 
                       err: ErrorCode,
                       decrypted_msg_raw: *const u8, 
                       decrypted_msg_len: u32))
        </pre>
    </td>
    <td>
<pre>
indy_crypto_auth_decrypt(
                command_handle: i32,
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
    <th colspan="2">
       Encrypts a message by anonymous-encryption scheme.
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_crypto_box_seal(
                command_handle: i32,
                their_vk: *const c_char,
                message_raw: *const u8,
                message_len: u32,
                cb: fn(xcommand_handle: i32, 
                       err: ErrorCode,
                       encrypted_msg_raw: *const u8, 
                       encrypted_msg_len: u32))
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
    <th colspan="2">
       Decrypts a message by anonymous-encryption scheme.
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_crypto_box_seal_open(
                command_handle: i32,
                wallet_handle: i32,
                my_vk: *const c_char,
                encrypted_msg_raw: *const u8,
                encrypted_msg_len: u32,
                cb: fn(xcommand_handle: i32, 
                       err: ErrorCode,
                       decrypted_msg_raw: *const u8, 
                       decrypted_msg_len: u32))
        </pre>
    </td>
    <td>
      <pre>
indy_crypto_anon_decrypt(
                command_handle: i32,
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
  <tr> 
      <th colspan="2">
        Creates keys pair and stores in the wallet.
      </th>
    </tr>
    <tr>
      <td>
        <pre>
  indy_create_key(...)
                </pre>
      </td>
      <td>
        <b>=</b>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
        Saves/replaces the meta information for the giving key in the wallet.
      </th>
    </tr>
    <tr>
      <td>
        <pre>
  indy_set_key_metadata(...)
                </pre>
      </td>
      <td>
        <b>=</b>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
        Retrieves the meta information for the giving key in the wallet.
      </th>
    </tr>
    <tr>
      <td>
        <pre>
  indy_get_key_metadata(...)
                </pre>
      </td>
      <td>
        <b>=</b>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
        Signs a message with a key.
      </th>
    </tr>
    <tr>
      <td>
        <pre>
  indy_crypto_sign(...)
                </pre>
      </td>
      <td>
        <b>=</b>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
         Verify a signature with a verkey.
      </th>
    </tr>
    <tr>
      <td>
        <pre>
  indy_crypto_verify(...)
                </pre>
      </td>
      <td>
        <b>=</b>
      </td>
    </tr>
</table>

### Agent API mapping
The Agent API file was deleted from Libindy but his functionality was simplified and moved to Crypto API.

<table>  
  <th>v1.3.0 - Agent API</th>
  <th>v1.4.0 - Crypto API</th>
  <tr> 
    <th colspan="2">
      Encrypt a message by authenticated-encryption scheme
    </th>
  </tr>
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
indy_crypto_auth_crypt(
                command_handle: i32,
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
    <th colspan="2">
      Encrypts a message by anonymous-encryption scheme.
    </th>
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
    <th colspan="2">
      Decrypts a message.
    </th>
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
      </re>
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
