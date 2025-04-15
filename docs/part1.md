# Part1

## 1. macro - account

### 1.1 #[account]

1. Explanation:
    ```#[account]``` is procedural macro which generates codes from raw codes under its function. It includes:

    - serilize struct type data as data blob and deserilize reversely

    - print struct in debug mode

2. Demo

    ```rust
    #[account]
    pub struct MyStorage {
        value1: u8,
    }
    ```
    - The #[account] macro will auto-generate the implementations for all of these traits: 
        - ```AccountSerialize```

        - ```AccountDeserialize```

        - ```Owner```

        - ```Clone```

        Those traits implementation is also used for trait bounds checking.
<br>
    - Based on struct ```MyStorage``` and macro ```#[account]```, three functions can be realized:

        - Serialize a MyStorage instance into bytes

        - Deserialize from bytes

        - Print it for debug

        ```rust
        fn main() {
            // ✅ Create an instance
            let storage = MyStorage { value1: 42 };

            // ✅ Serialize with discriminator
            let mut serialized: Vec<u8> = Vec::new();
            storage.try_serialize(&mut serialized).unwrap();

            // ✅ Deserialize from bytes
            let mut bytes_slice = serialized.as_slice();
            let deserialized = MyStorage::try_deserialize(&mut bytes_slice).unwrap();

            // ✅ Print result
            println!("Deserialized struct: {:?}", deserialized);
        }
        ```

### 1.2 #[derive(Accounts)]

#### 1.2.1 <'info>

1. Explanation:
    ```<'info>``` is a lifetime parameter and acts like a binding checker:

    - It ensures the memory that references point to is valid and stable for as long as needed.

    - It prevents use-after-free, dangling references, or accidental data invalidation.

2. Demo

```rust
#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,
}

/// definition of Account<'info, T>
#[derive(Clone)]
pub struct Account<'info, T: AccountSerialize + AccountDeserialize + Clone> {
    account: T,
    info: &'info AccountInfo<'info>,
}

/// Account information
#[derive(Clone)]
#[repr(C)]
pub struct AccountInfo<'a> {
    /// Public key of the account
    pub key: &'a Pubkey,
    /// The lamports in the account.  Modifiable by programs.
    pub lamports: Rc<RefCell<&'a mut u64>>,
    /// The data held in this account.  Modifiable by programs.
    pub data: Rc<RefCell<&'a mut [u8]>>,
    /// Program that owns this account
    pub owner: &'a Pubkey,
    /// The epoch at which this account will next owe rent
    pub rent_epoch: Epoch,
    /// Was the transaction signed by this account's public key?
    pub is_signer: bool,
    /// Is the account writable?
    pub is_writable: bool,
    /// This account's data contains a loaded program (and is now read-only)
    pub executable: bool,
}
```
- How ```<'info>``` works in demo code
    The data ```Account<'info, MyStorage>``` references is all the information under ```AccountInfo<'info>``` which includes referenced or borrowed value. <'info> passed through from outside all the way to ```AccountInfo``` is to make sure the memories from outside reference is binded validly.

<br>

- Why it works this way:
    - The Solana runtime gives you a set of ```AccountInfo```s at the start of an instruction.

    - Each ```AccountInfo<'info>``` contains borrowed slices and references (not owned values).

    - Anchor wraps this into ```Account<'info, T>``` by deserializing the data part into ```T```.

    - Rust’s borrow checker needs to track how long those references are valid — that’s what 'info does.